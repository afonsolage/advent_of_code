#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn is_operational(&self) -> bool {
        matches!(self, Spring::Operational)
    }
    fn is_damaged(&self) -> bool {
        matches!(self, Spring::Damaged)
    }
    fn is_unknown(&self) -> bool {
        matches!(self, Spring::Unknown)
    }
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        };
        write!(f, "{}", c)
    }
}

fn reduce(registry: &mut [Spring], records: &[usize]) -> (usize, u64) {
    if records.len() == 1 {
        // Can't be reduced even further.
        let is_damaged_remaining = registry.iter().any(|c| c.is_damaged());
        if is_damaged_remaining {
            // If there are still damaged springs remaining on registry
            // this means the registry is invalid.
            (0, 0)
        } else {
            (0, 1)
        }
    } else if registry.first() == Some(&Spring::Unknown) {
        registry[0] = Spring::Operational;
        let res = count_arrangements(registry, &records[1..]);
        registry[0] = Spring::Unknown;
        res
    } else {
        count_arrangements(registry, &records[1..])
    }
}

fn count_arrangements(registry: &mut [Spring], records: &[usize]) -> (usize, u64) {
    // print!("Count: ");
    // registry.iter().for_each(|s| print!("{}", s));
    // records.iter().for_each(|r| print!(" {}", r));
    // println!();

    if registry.is_empty() || records.is_empty() {
        return (0, 0);
    }

    let record = *records.first().unwrap();

    let skip_operational = registry.iter().take_while(|c| c.is_operational()).count();
    // dbg!(skip_operational);
    let damaged_count = registry
        .iter()
        .skip(skip_operational)
        .take_while(|c| c.is_damaged())
        .count();

    // dbg!(damaged_count);

    if damaged_count > record {
        // If there are more damaged springs than our recorded count
        // this means this registry is invalid.
        return (0, 0);
    }

    let next_pos = skip_operational + damaged_count;

    if damaged_count == record {
        let (reduced_pos, reduced_arrangements) = reduce(&mut registry[next_pos..], records);
        return (next_pos + reduced_pos, reduced_arrangements);
    }

    // At this point, the following is true:
    // - next_pos is either '.' or '?'
    // - damaged_count is lesser than record

    let Some(next_spring) = registry.get(next_pos) else {
        // End of registry reached, registry is invalid
        return (0, 0);
    };

    if next_spring.is_operational() {
        // Damaged count doesn't match record. Invalid registry
        return (0, 0);
    }

    assert!(next_spring.is_unknown());
    // At this point, next_spring is unknown

    if damaged_count > 0 && damaged_count < record {
        // There is at least one damaged spring.
        // This spring must be a damaged one.
        registry[next_pos] = Spring::Damaged;
        let res = count_arrangements(registry, records);
        registry[next_pos] = Spring::Unknown;
        return res;
    }

    // At this point unknown can be either Operational or Damaged
    registry[next_pos] = Spring::Operational;
    let (op_pos, op_arrangements) = count_arrangements(registry, records);
    registry[next_pos] = Spring::Damaged;
    let (dmg_pos, dmg_arrangements) = count_arrangements(registry, records);

    // Restore unknown value
    registry[next_pos] = Spring::Unknown;

    if op_arrangements > 0 && dmg_arrangements > 0 {
        (op_pos.min(dmg_pos), op_arrangements + dmg_arrangements)
    } else if op_arrangements > 0 {
        (op_pos, op_arrangements)
    } else if dmg_arrangements > 0 {
        (dmg_pos, dmg_arrangements)
    } else {
        (0, 0)
    }
}

fn part01(input: &str) -> u64 {
    let count = input.lines().count();
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (registry, records) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();

            println!("## {registry} ({i}/{count}) ##");
            let records = records
                .split(',')
                .map(|r| r.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let mut registry = registry.chars().map(|c| c.into()).collect::<Vec<_>>();

            let (_, count) = count_arrangements(&mut registry, &records);
            count
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
    const INPUT_SINGLE: &str = "?#?#?#?#?#?#?#? 1,3,1,6";
    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part01_single() {
        assert_eq!(super::part01(INPUT_SINGLE), 1);
    }

    #[test]
    fn part01() {
        assert_eq!(super::part01(INPUT), 21);
    }

    // #[test]
    // fn part02() {
    //     assert_eq!(super::part02(INPUT), 525152);
    // }
    //

    #[test]
    fn count_arrangements_simple() {
        assert_eq!(1, super::part01("### 3"))
    }
}

