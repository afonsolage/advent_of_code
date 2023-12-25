fn traverse(
    registry: &mut [char],
    pos: usize,
    valid_registries: &mut Vec<Vec<char>>,
    records: &[usize],
) {
    if let Some(&c) = registry.get(pos) {
        match c {
            '.' | '#' => traverse(registry, pos + 1, valid_registries, records),
            '?' => {
                let mut f_registry = registry.to_vec();
                f_registry[pos] = '.';
                // if is_partial_registry_valid(&f_parent, records) {
                traverse(&mut f_registry, pos + 1, valid_registries, records);
                // }

                let d_registry = registry;
                d_registry[pos] = '#';
                // if is_partial_registry_valid(&d_parent, records) {
                traverse(d_registry, pos + 1, valid_registries, records);
                // }
            }
            _ => unreachable!(),
        }
    } else if is_registry_valid(registry, records) {
        valid_registries.push(registry.to_vec());
    }
}

// fn is_partial_registry_valid(registry: &str, record: &[usize]) -> bool {
//     println!("Is partial registry valid: {registry}");
//     let (partial_record, _) = registry.split_once('?').unwrap();
//
//     partial_record
//         .split('.')
//         .filter(|s| !s.is_empty())
//         .zip(record)
//         .all(|(rec, &reg)| rec.len() <= reg)
// }

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
    input
        .lines()
        .map(|line| {
            let (registry, records) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();

            let records = records
                .split(',')
                .map(|r| r.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut parsed_registries = vec![];
            println!("Traversing: {registry}");
            let mut registry = registry.chars().collect::<Vec<_>>();
            traverse(&mut registry, 0, &mut parsed_registries, &records);

            parsed_registries
                .into_iter()
                .filter(|reg| is_registry_valid(reg, &records))
                .count() as u64
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

