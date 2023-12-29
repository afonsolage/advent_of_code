fn hash(input: &str) -> u64 {
    input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .fold(0, |acc, c| ((acc + (c as u8) as u64) * 17) % 256)
}

fn part01(input: &str) -> u64 {
    input.split(',').map(hash).sum()
}

fn part02(input: &str) -> u64 {
    let input = input.trim_matches(|c: char| c.is_ascii_whitespace());
    let mut map: Vec<Vec<(&str, u64)>> = vec![vec![]; 256];

    input.split(',').for_each(|step| {
        if let Some(label) = step.strip_suffix('-') {
            let hash = hash(label);
            map[hash as usize].retain(|pair| pair.0 != label);
        } else {
            let (label, focal_len) = step.split_once('=').unwrap();
            let hash = hash(label);
            let focal_len = focal_len.parse::<u64>().unwrap();
            if let Some(pair) = map[hash as usize].iter_mut().find(|pair| pair.0 == label) {
                pair.1 = focal_len
            } else {
                map[hash as usize].push((label, focal_len));
            }
        }
    });

    map.into_iter()
        .enumerate()
        .flat_map(|(box_idx, box_content)| {
            box_content
                .into_iter()
                .enumerate()
                .map(move |(slot, pair)| (box_idx as u64 + 1) * (slot as u64 + 1) * pair.1)
        })
        .sum()
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(super::part02(input), 145);
    }
}

