use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
    #[default]
    Low,
    High,
}

impl Pulse {
    fn is_low(&self) -> bool {
        matches!(self, Pulse::Low)
    }

    fn is_high(&self) -> bool {
        matches!(self, Pulse::High)
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_low() {
            write!(f, "low")
        } else {
            write!(f, "high")
        }
    }
}

impl std::ops::Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self.is_low() {
            Pulse::High
        } else {
            Pulse::Low
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
    Button,
    #[default]
    Untyped,
}

#[derive(Default, Debug, Clone)]
struct Module {
    name: String,
    state: Vec<(String, Pulse)>,
    destinations: Vec<String>,
    tp: ModuleType,
}

impl Module {
    fn receive(&mut self, sender: &str, pulse: Pulse) -> Option<Pulse> {
        match self.tp {
            ModuleType::FlipFlop if pulse.is_low() => {
                self.state[0].1 = !self.state[0].1;
                return Some(self.state[0].1);
            }
            ModuleType::Conjunction => {
                let state = self.state.iter_mut().find(|(s, _)| s == sender).unwrap();
                state.1 = pulse;

                if self.state.iter().all(|(_, p)| p.is_high()) {
                    return Some(Pulse::Low);
                } else {
                    return Some(Pulse::High);
                }
            }
            ModuleType::Broadcaster | ModuleType::Button => {
                return Some(pulse);
            }
            _ => (),
        }

        None
    }
}

fn parse_modules(input: &str) -> HashMap<String, Module> {
    let mut modules = input
        .lines()
        .map(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();

            let (tp, name) = if let Some(name) = name.strip_prefix('%') {
                (ModuleType::FlipFlop, name.to_string())
            } else if let Some(name) = name.strip_prefix('&') {
                (ModuleType::Conjunction, name.to_string())
            } else if name == "broadcaster" {
                (ModuleType::Broadcaster, name.to_string())
            } else {
                (ModuleType::Untyped, name.to_string())
            };

            let destinations = destinations
                .split(',')
                .map(|name| name.trim().to_string())
                .collect();

            let module = Module {
                tp,
                name: name.clone(),
                destinations,
                ..Default::default()
            };

            (name, module)
        })
        .collect::<HashMap<_, _>>();

    set_internal_state(&mut modules);

    let output = Module {
        name: "output".to_string(),
        tp: ModuleType::Untyped,
        ..Default::default()
    };

    let button = Module {
        name: "button".to_string(),
        tp: ModuleType::Button,
        destinations: vec!["broadcast".to_string()],
        state: Default::default(),
    };

    modules.insert("output".to_string(), output);
    modules.insert("button".to_string(), button);

    modules
}

fn set_internal_state(modules: &mut HashMap<String, Module>) {
    let inputs = modules
        .values()
        .filter(|m| matches!(m.tp, ModuleType::Conjunction))
        .map(|conj| {
            let input_count = modules
                .values()
                .filter_map(|m| {
                    if m.destinations.contains(&conj.name) {
                        Some((m.name.clone(), Pulse::default()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            (conj.name.clone(), input_count)
        })
        .collect::<Vec<_>>();

    modules.values_mut().for_each(|module| match module.tp {
        ModuleType::FlipFlop => {
            module.state = vec![Default::default()];
        }
        ModuleType::Conjunction => {
            module.state = inputs
                .iter()
                .find_map(|(n, s)| {
                    if n == &module.name {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .unwrap();
        }
        _ => (),
    })
}

fn push_button(modules: &mut HashMap<String, Module>) -> (u64, u64) {
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    let mut queue = VecDeque::new();
    queue.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));

    while let Some((sender, pulse, target)) = queue.pop_front() {
        if pulse.is_low() {
            low_pulse_count += 1;
        } else {
            high_pulse_count += 1;
        }

        let Some(module) = modules.get_mut(&target) else {
            continue;
        };

        if let Some(pulse) = module.receive(&sender, pulse) {
            for dest in &module.destinations {
                queue.push_back((target.clone(), pulse, dest.clone()));
            }
        }
    }

    (low_pulse_count, high_pulse_count)
}

fn part01(input: &str) -> u64 {
    let mut modules = parse_modules(input);
    let (mut low, mut high) = (0, 0);

    for _ in 0..1000 {
        let (l, h) = push_button(&mut modules);
        low += l;
        high += h;
    }

    low * high
}

fn predict_rx_push_button_count(modules: &mut HashMap<String, Module>) -> Vec<u64> {
    let parent = modules
        .values()
        .find(|m| m.destinations.first().is_some_and(|d| d == "rx"))
        .unwrap();

    assert!(matches!(parent.tp, ModuleType::Conjunction));

    let mut targets = modules
        .values()
        .filter_map(|m| {
            if m.destinations.contains(&parent.name) {
                Some((m.name.clone(), 0u64))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut btn_press_cnt = 0;

    loop {
        btn_press_cnt += 1;
        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));

        while let Some((sender, pulse, target)) = queue.pop_front() {
            if pulse.is_high() {
                if let Some(target_cnt) =
                    targets
                        .iter_mut()
                        .find_map(|(m, cnt)| if m == &sender { Some(cnt) } else { None })
                {
                    if *target_cnt == 0 {
                        *target_cnt = btn_press_cnt;

                        if targets.iter().all(|(_, cnt)| cnt > &0) {
                            return targets.into_iter().map(|(_, cnt)| cnt).collect();
                        }
                    }
                }
            }

            let Some(module) = modules.get_mut(&target) else {
                continue;
            };

            if let Some(pulse) = module.receive(&sender, pulse) {
                for dest in &module.destinations {
                    queue.push_back((target.clone(), pulse, dest.clone()));
                }
            }
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn part02(input: &str) -> u64 {
    let mut modules = parse_modules(input);
    predict_rx_push_button_count(&mut modules)
        .into_iter()
        .fold(1, lcm)
}

fn main() {
    let input = include_str!("../input/day20.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01_1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        assert_eq!(super::part01(input), 32000000);
    }

    #[test]
    fn part01_2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        assert_eq!(super::part01(input), 11687500);
    }
}

