fn part01(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut digits = l.chars().filter(char::is_ascii_digit);
            let Some(first) = digits.next() else {
                return 0;
            };

            format!("{}{}", first, digits.last().unwrap_or(first))
                .parse::<u32>()
                .unwrap_or_default()
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
enum ParsedNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Digit(u32),
}

impl ParsedNumber {
    const DIGITS: [&'static str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn try_parse(input: &str) -> Option<ParsedNumber> {
        if let Some(c) = input.chars().next() {
            if c.is_ascii_digit() {
                return Some(Self::Digit(c.to_digit(10).unwrap()));
            }
        }

        for digit in Self::DIGITS {
            if input.starts_with(digit) {
                return Self::from_str(digit);
            }
        }

        None
    }

    fn from_str(input: &str) -> Option<Self> {
        match input {
            "one" => Some(ParsedNumber::One),
            "two" => Some(ParsedNumber::Two),
            "three" => Some(ParsedNumber::Three),
            "four" => Some(ParsedNumber::Four),
            "five" => Some(ParsedNumber::Five),
            "six" => Some(ParsedNumber::Six),
            "seven" => Some(ParsedNumber::Seven),
            "eight" => Some(ParsedNumber::Eight),
            "nine" => Some(ParsedNumber::Nine),
            _ => None,
        }
    }

    fn number(&self) -> u32 {
        match *self {
            ParsedNumber::Digit(n) => n,
            ParsedNumber::One => 1,
            ParsedNumber::Two => 2,
            ParsedNumber::Three => 3,
            ParsedNumber::Four => 4,
            ParsedNumber::Five => 5,
            ParsedNumber::Six => 6,
            ParsedNumber::Seven => 7,
            ParsedNumber::Eight => 8,
            ParsedNumber::Nine => 9,
        }
    }
}

fn part02(input: &str) -> u32 {
    input.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> u32 {
    let mut index = 0;
    let mut parsed_digits = vec![];

    loop {
        let input = &line[index..];
        index += 1;

        if input.is_empty() {
            break;
        }

        if let Some(parsed) = ParsedNumber::try_parse(input) {
            parsed_digits.push(parsed);
        }
    }

    if parsed_digits.is_empty() {
        return 0;
    }

    let first = parsed_digits.first().unwrap();
    let second = parsed_digits.last().unwrap();

    format!("{}{}", first.number(), second.number())
        .parse::<u32>()
        .unwrap_or_default()
}

fn main() {
    let input = include_str!("../input/day01.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(super::part01(input), 142);
    }

    #[test]
    fn part02() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(super::part02(input), 281)
    }
}
