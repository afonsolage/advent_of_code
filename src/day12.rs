#[derive(Debug)]
enum State {
    Functional(Option<Box<State>>),
    Damaged(Option<Box<State>>),
    Unknown(Option<Box<State>>),
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => State::Functional(None),
            '#' => State::Damaged(None),
            '?' => State::Unknown(None),
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let next = match self {
            State::Functional(n) => {
                write!(f, ".")?;
                n
            }
            State::Damaged(n) => {
                write!(f, "#")?;
                n
            }
            State::Unknown(n) => {
                write!(f, "?")?;
                n
            }
        };
        if let Some(next) = next {
            write!(f, "{next}")
        } else {
            Ok(())
        }
    }
}

fn parse(line: &[char]) -> Option<Box<State>> {
    if line.is_empty() {
        return None;
    }

    let mut state: Box<State> = Box::new(line[0].into());

    let next = if line.len() > 1 {
        parse(&line[1..])
    } else {
        None
    };

    match state.as_mut() {
        State::Functional(ref mut n) | State::Damaged(ref mut n) | State::Unknown(ref mut n) => {
            *n = next
        }
    }

    Some(state)
}

fn part01(input: &str) -> u64 {
    let lines = input
        .lines()
        .map(|line| {
            let (registry, records) = line.split_once(|c: char| c.is_ascii_whitespace()).unwrap();
            (parse(&registry.chars().collect::<Vec<_>>()), records)
        })
        .collect::<Vec<_>>();

    for line in lines {
        if let (Some(state), records) = line {
            println!("{state} {records:?}");
            println!();
        }
    }

    0
}

fn part02(_input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input/day12.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(super::part01(input), 21);
    }

    #[test]
    fn part02() {
        let input = "";

        assert_eq!(super::part02(input), 0);
    }
}

