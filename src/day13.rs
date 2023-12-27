fn count_mirroed_left_line(line: &[u64]) -> u64 {
    (1..(line.len() - 1))
        .map(|mid| line.split_at(mid))
        .find_map(|(left, right)| {
            let min_len = left.len().min(right.len());
            let last = left.len() - 1;
            if (0..min_len).all(|idx| left[last - idx] == right[idx]) {
                Some(left.len() as u64)
            } else {
                None
            }
        })
        .unwrap_or(0)
}

fn parse_line(segment: &str) -> Vec<u64> {
    segment
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => unreachable!(),
                })
                .enumerate()
        })
        .enumerate()
        .fold(vec![], |mut acc, (offset, iter)| {
            iter.for_each(|(idx, n)| {
                if idx < acc.len() {
                    acc[idx] |= n << offset;
                } else {
                    acc.push(n);
                }
            });
            acc
        })
}

fn parse_column(segment: &str) -> Vec<u64> {
    segment
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => unreachable!(),
                })
                .enumerate()
        })
        .enumerate()
        .fold(vec![], |mut acc, (idx, iter)| {
            iter.for_each(|(offset, n)| {
                if idx < acc.len() {
                    acc[idx] |= n << offset;
                } else {
                    acc.push(n);
                }
            });
            acc
        })
}

fn part01(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|segment| (parse_line(segment), parse_column(segment)))
        .map(|(line, column)| {
            let line_cnt = count_mirroed_left_line(&line);
            let column_cnt = count_mirroed_left_line(&column);
            line_cnt + column_cnt * 100
        })
        .sum()
}

fn part02(_input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input/day13.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn parse_line() {
        assert_eq!(
            super::parse_line(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            ),
            vec![77, 12, 115, 33, 82, 82, 33, 115, 12]
        );
    }

    #[test]
    fn parse_column() {
        assert_eq!(
            super::parse_column(
                "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            vec![305, 289, 460, 223, 223, 460, 289]
        )
    }

    #[test]
    fn count_mirroed_left_line() {
        assert_eq!(
            super::count_mirroed_left_line(&[77, 12, 115, 33, 82, 82, 33, 115, 12]),
            5
        )
    }

    #[test]
    fn part01() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(super::part01(input), 405);
    }

    // #[test]
    // fn part02() {
    //     let input = "";
    //
    //     assert_eq!(super::part02(input), 0);
    // }
}

