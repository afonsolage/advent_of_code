fn predict(line: &str) -> i64 {
    let values = line
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut interpolation = values.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    let mut interpolations = vec![interpolation];

    loop {
        interpolation = interpolations
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();

        if interpolation.iter().all(|&v| v == 0) {
            break;
        }

        interpolations.push(interpolation);
    }

    let diff = interpolations
        .into_iter()
        .rev()
        .map(|interpolation| *interpolation.last().unwrap())
        .sum::<i64>();

    diff + values.last().unwrap()
}

fn predict_back(line: &str) -> i64 {
    let values = line
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut interpolation = values.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    let mut interpolations = vec![interpolation];

    loop {
        interpolation = interpolations
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[0] - w[1])
            .collect::<Vec<_>>();

        if interpolation.iter().all(|&v| v == 0) {
            break;
        }

        interpolations.push(interpolation);
    }

    let diff = interpolations
        .into_iter()
        .rev()
        .map(|interpolation| *interpolation.first().unwrap())
        .sum::<i64>();

    values.first().unwrap() - diff
}

fn part01(input: &str) -> i64 {
    input.lines().map(|line| predict(line)).sum()
}

fn part02(input: &str) -> i64 {
    input.lines().map(|line| predict_back(line)).sum()
}

fn main() {
    let input = include_str!("../input/day09.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn predict_line_1() {
        assert_eq!(super::predict("0 3 6 9 12 15"), 18);
    }

    #[test]
    fn predict_line_2() {
        assert_eq!(super::predict("1 3 6 10 15 21"), 28);
    }

    #[test]
    fn predict_line_3() {
        assert_eq!(super::predict("10 13 16 21 30 45"), 68);
    }

    #[test]
    fn predict_back_line_1() {
        assert_eq!(super::predict_back("0 3 6 9 12 15"), -3);
    }

    #[test]
    fn predict_back_line_2() {
        assert_eq!(super::predict_back("1 3 6 10 15 21"), 0);
    }

    #[test]
    fn predict_back_line_3() {
        assert_eq!(super::predict_back("10 13 16 21 30 45"), 5);
    }

    #[test]
    fn part01() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        assert_eq!(super::part01(input), 114);
    }

    #[test]
    fn part02() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        assert_eq!(super::part02(input), 2);
    }
}
