use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Node<'a>(&'a str, &'a str);

fn part01(input: &str) -> u64 {
    let instructions = input
        .split("\n\n")
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    let nodes = input
        .lines()
        .skip(2)
        .map(|line| {
            let (key, lr) = line.split_once('=').unwrap();
            let key = key.trim();
            let (left, right) = lr
                .trim()
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(',')
                .unwrap();

            (key, Node(left.trim(), right.trim()))
        })
        .collect::<HashMap<_, _>>();

    let mut steps = 0;
    let mut next_instruction = 0;
    let mut next_node = "AAA";

    loop {
        steps = steps + 1;

        let instruction = instructions[next_instruction];
        next_instruction += 1;
        next_instruction %= instructions.len();

        let node = nodes.get(next_node).unwrap();
        next_node = match instruction {
            'L' => node.0,
            'R' => node.1,
            _ => unreachable!(),
        };

        if next_node == "ZZZ" {
            return steps;
        }
    }
}

fn part02(_input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input/day08.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        assert_eq!(super::part01(input), 2);
    }

    #[test]
    fn part01_repeat() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        assert_eq!(super::part01(input), 6);
    }

    #[test]
    fn part02() {
        let input = "";

        assert_eq!(super::part02(input), 0);
    }
}
