fn hash(input: &str) -> u64 {
    input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .fold(0, |acc, c| ((acc + (c as u8) as u64) * 17) % 256)
}

fn part01(input: &str) -> u64 {
    input.split(',').map(hash).sum()
}

fn part02(_input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input/day15.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn hash() {
        assert_eq!(super::hash("HASH"), 52);
    }

    #[test]
    fn part01() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(super::part01(input), 1320);
    }

    #[test]
    fn part02() {
        let input = "";

        assert_eq!(super::part02(input), 0);
    }
}

