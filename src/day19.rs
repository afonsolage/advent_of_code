use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Action<'a> {
    Accept,
    Reject,
    Goto(&'a str),
}
impl Action<'_> {
    fn parse(step: &str) -> Action {
        if step == "R" {
            Action::Reject
        } else if step == "A" {
            Action::Accept
        } else {
            Action::Goto(step)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Prop {
    X,
    M,
    A,
    S,
}
impl Prop {
    fn parse(prop: char) -> Prop {
        match prop {
            'x' => Prop::X,
            'm' => Prop::M,
            'a' => Prop::A,
            's' => Prop::S,
            _ => unreachable!(),
        }
    }

    fn get(&self, part: &Part) -> u32 {
        match self {
            Prop::X => part.x,
            Prop::M => part.m,
            Prop::A => part.a,
            Prop::S => part.s,
        }
    }

    fn set_min(&self, part: PartRange, value: u32) -> PartRange {
        match self {
            Prop::X => PartRange {
                x: (value, part.x.1),
                ..part
            },
            Prop::M => PartRange {
                m: (value, part.m.1),
                ..part
            },
            Prop::A => PartRange {
                a: (value, part.a.1),
                ..part
            },
            Prop::S => PartRange {
                s: (value, part.s.1),
                ..part
            },
        }
    }

    fn set_max(&self, part: PartRange, value: u32) -> PartRange {
        match self {
            Prop::X => PartRange {
                x: (part.x.0, value),
                ..part
            },
            Prop::M => PartRange {
                m: (part.m.0, value),
                ..part
            },
            Prop::A => PartRange {
                a: (part.a.0, value),
                ..part
            },
            Prop::S => PartRange {
                s: (part.s.0, value),
                ..part
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Lt(Prop, u32),
    Gt(Prop, u32),
}

impl Op {
    fn parse(op: &str) -> Op {
        let prop = op.chars().next().unwrap();
        let operator = op.chars().nth(1).unwrap();
        let value = (op[2..]).parse().unwrap();

        if operator == '>' {
            Op::Gt(Prop::parse(prop), value)
        } else {
            Op::Lt(Prop::parse(prop), value)
        }
    }

    fn check(&self, part: &Part) -> bool {
        match self {
            Op::Lt(prop, value) => prop.get(part) < *value,
            Op::Gt(prop, value) => prop.get(part) > *value,
        }
    }

    fn slice(&self, part: PartRange) -> (PartRange, PartRange) {
        match self {
            Op::Lt(prop, value) => (prop.set_max(part, value - 1), prop.set_min(part, *value)),
            Op::Gt(prop, value) => (prop.set_min(part, value + 1), prop.set_max(part, *value)),
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow<'a> {
    steps: Vec<(Option<Op>, Action<'a>)>,
}

#[derive(Default, Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct PartRange {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl PartRange {
    fn full() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn combinations(&self) -> u64 {
        ((self.x.1 - self.x.0) as u64 + 1)
            * ((self.m.1 - self.m.0) as u64 + 1)
            * ((self.a.1 - self.a.0) as u64 + 1)
            * ((self.s.1 - self.s.0) as u64 + 1)
    }
}

fn process(workflows: &HashMap<&str, Workflow>, part: Part) -> bool {
    let mut name = "in";
    loop {
        let workflow = workflows.get(name).unwrap();
        for (maybe_op, action) in &workflow.steps {
            let execute_action = if let Some(op) = maybe_op {
                op.check(&part)
            } else {
                true
            };

            if execute_action {
                match action {
                    Action::Accept => {
                        return true;
                    }
                    Action::Reject => {
                        return false;
                    }
                    Action::Goto(step) => {
                        name = step;
                        break;
                    }
                }
            }
        }
    }
}

fn count_combinations(
    starting_name: &str,
    range: PartRange,
    workflows: &HashMap<&str, Workflow>,
) -> u64 {
    let workflow = workflows.get(starting_name).unwrap();

    let (combinations, _) =
        workflow
            .steps
            .iter()
            .fold((0, range), |(combinations, range), (maybe_op, action)| {
                let (left, maybe_right) = if let Some(op) = maybe_op {
                    let (left, right) = op.slice(range);
                    (left, Some(right))
                } else {
                    (range, None)
                };

                let left_combinations = match action {
                    Action::Accept => left.combinations(),
                    Action::Reject => 0,
                    Action::Goto(next) => count_combinations(next, left, workflows),
                };

                let range = maybe_right.unwrap_or(range);

                (combinations + left_combinations, range)
            });

    combinations
}

fn part01(input: &str) -> u64 {
    let workflows = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let rest = rest.trim_end_matches('}');
            let steps = rest
                .split(',')
                .map(|step| {
                    if step.contains(':') {
                        let (op, action) = step.split_once(':').unwrap();
                        (Some(Op::parse(op)), Action::parse(action))
                    } else {
                        (None, Action::parse(step))
                    }
                })
                .collect();
            (name, Workflow { steps })
        })
        .collect::<HashMap<_, _>>();

    let parts = input
        .lines()
        .skip(workflows.len() + 1)
        .map(|line| {
            let line = &line[1..line.len() - 1];
            line.split(',').fold(Part::default(), |mut acc, prop| {
                let (prop, value) = prop.split_once('=').unwrap();
                let value = value.parse().unwrap();
                match prop {
                    "x" => acc.x = value,
                    "m" => acc.m = value,
                    "a" => acc.a = value,
                    "s" => acc.s = value,
                    _ => unreachable!(),
                }
                acc
            })
        })
        .collect::<Vec<_>>();

    parts
        .into_iter()
        .map(|part| {
            if process(&workflows, part) {
                part.sum() as u64
            } else {
                0
            }
        })
        .sum()
}

fn part02(input: &str) -> u64 {
    let workflows = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let rest = rest.trim_end_matches('}');
            let steps = rest
                .split(',')
                .map(|step| {
                    if step.contains(':') {
                        let (op, action) = step.split_once(':').unwrap();
                        (Some(Op::parse(op)), Action::parse(action))
                    } else {
                        (None, Action::parse(step))
                    }
                })
                .collect();
            (name, Workflow { steps })
        })
        .collect::<HashMap<_, _>>();

    count_combinations("in", PartRange::full(), &workflows)
}

fn main() {
    let input = include_str!("../input/day19.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {
    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part01() {
        assert_eq!(super::part01(INPUT), 19114);
    }

    #[test]
    fn part02() {
        assert_eq!(super::part02(INPUT), 167409079868000);
    }
}

