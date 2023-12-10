use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Node<'a>(&'a str, &'a str);

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

fn gcd(a: u128, b: u128) -> u128 {
    let mut max = a;
    let mut min = b;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn steps_count(instructions: Vec<char>, nodes: HashMap<&str, Node>, first_node: &str) -> u128 {
    let mut steps = 0;
    let mut next_instruction = 0;
    let mut next_node = first_node;

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

        if next_node.ends_with("Z") {
            return steps;
        }
    }
}

fn part01(input: &str) -> u128 {
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

    steps_count(instructions, nodes, "AAA")
}

fn part02(input: &str) -> u128 {
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

    let count = nodes
        .keys()
        .copied()
        .filter(|k| k.ends_with("A"))
        .map(|node| steps_count(instructions.clone(), nodes.clone(), node))
        .collect::<Vec<_>>();

    count
        .into_iter()
        .fold(1, |product, count| lcm(product, count))
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

        assert_eq!(super::part02(input), 2);
    }

    #[test]
    fn part01_repeat() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        assert_eq!(super::part02(input), 6);
    }

    #[test]
    fn part02() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        assert_eq!(super::part02(input), 6);
    }
}
