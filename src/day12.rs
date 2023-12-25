use std::collections::HashSet;

#[derive(Default, Debug, Clone)]
struct Node {
    registry: Vec<char>,
    pos: usize,
}

impl Node {
    fn get(&self) -> Option<&char> {
        self.registry.get(self.pos)
    }

    fn set(&mut self, val: char) {
        self.registry[self.pos] = val;
    }

    fn next(&mut self) {
        self.pos += 1;
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}){}",
            self.pos,
            String::from_iter(self.registry.iter())
        )
    }
}

fn traverse(registry: Vec<char>, records: &[usize]) -> usize {
    let mut count = 0;
    let mut stack = vec![Node { registry, pos: 0 }];
    let mut add_node: Option<Node> = None;

    // let mut log_count = 0;

    let mut set = HashSet::new();

    while !stack.is_empty() {
        if let Some(add_node) = add_node.take() {
            let str = String::from_iter(add_node.registry.iter());
            if !set.contains(&str) {
                set.insert(str);
            } else {
                println!("Duplicated! {str}");
            }
            stack.push(add_node);
        }
        // let len = stack.len();
        let node = stack.last_mut().unwrap();

        if let Some(&c) = node.get() {
            match c {
                '.' | '#' => node.next(),
                '?' => {
                    node.set('.');
                    let is_maybe_operational = is_partial_registry_valid(&node.registry, records);
                    node.set('#');
                    let is_maybe_damaged = is_partial_registry_valid(&node.registry, records);

                    if !is_maybe_operational && !is_maybe_damaged {
                        stack.pop();
                    } else if !is_maybe_operational {
                        node.set('#');
                    } else if !is_maybe_damaged {
                        node.set('.');
                    } else {
                        // println!("({len})Can be both {node}");
                        // log_count += 1;
                        // if log_count > 500 {
                        // return 0;
                        // }
                        let mut functional_node = node.clone();
                        functional_node.set('.');
                        add_node = Some(functional_node);

                        node.set('#');
                    }
                }
                _ => unreachable!(),
            }
        } else {
            if is_registry_valid(&node.registry, records) {
                count += 1;
            }
            stack.pop();
        }
    }

    count
}

fn is_partial_registry_valid(registry: &[char], record: &[usize]) -> bool {
    if let Some(index) = registry.iter().position(|&c| c == '?') {
        let (partial_record, _) = registry.split_at(index);
        let last_char = partial_record.iter().last().copied();

        let mut it = partial_record
            .split(|&c| c == '.')
            .filter(|s| !s.is_empty())
            .zip(record)
            .peekable();

        let tmp = partial_record.iter().collect::<String>();
        // println!("Checking {tmp} [{record:?}]");

        while let Some((reg, &rec)) = it.next() {
            if it.peek().is_none() {
                if Some('#') == last_char && reg.len() > rec {
                    return false;
                }
            } else if reg.len() != rec {
                return false;
            }
        }
    }

    true
}

fn is_registry_valid(registry: &[char], record: &[usize]) -> bool {
    let count = registry
        .split(|&c| c == '.')
        .filter(|s| !s.is_empty())
        .count();

    count == record.len()
        && registry
            .split(|&c| c == '.')
            .filter(|s| !s.is_empty())
            .zip(record)
            .all(|(rec, &reg)| rec.len() == reg)
}

fn part01(input: &str) -> u64 {
    let count = input.lines().count();
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (registry, records) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();

            let records = records
                .split(',')
                .map(|r| r.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            println!("Traversing: {registry} ({i}/{count})");
            let registry = registry.chars().collect::<Vec<_>>();
            traverse(registry, &records) as u64
        })
        .sum()
}

fn expand_line(input: &str) -> String {
    let (reg, rec) = input.split_once(|c: char| c.is_ascii_whitespace()).unwrap();

    let (expanded_reg, expanded_rec) = (0..5).fold(
        (String::new(), String::new()),
        |(mut acc_reg, mut acc_rec), n| {
            acc_reg.push_str(reg);
            acc_rec.push_str(rec);
            if n != 4 {
                acc_reg.push('?');
                acc_rec.push(',');
            }
            (acc_reg, acc_rec)
        },
    );

    format!("{expanded_reg} {expanded_rec}")
}

fn part02(input: &str) -> u64 {
    let expanded = input
        .lines()
        .map(expand_line)
        .fold(String::new(), |mut acc, line| {
            acc.push('\n');
            acc.push_str(&line);
            acc
        });
    part01(expanded.trim())
}

fn main() {
    let input = include_str!("../input/day12.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {
    // const INPUT_SINGLE: &str = "?#?#?#?#?#?#?#? 1,3,1,6";
    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part01() {
        assert_eq!(super::part01(INPUT), 21);
    }

    #[test]
    fn part02() {
        assert_eq!(super::part02(INPUT), 525152);
    }
}

