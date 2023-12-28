fn is_smudged(a: u64, b: u64) -> bool {
    let diff = a.abs_diff(b);
    diff != 0 && (diff & (diff - 1)) == 0
}

fn count_smudged(line: &[u64]) -> u64 {
    (1..(line.len()))
        .map(|mid| line.split_at(mid))
        .filter_map(|(left, right)| {
            let min_len = left.len().min(right.len());
            let last = left.len() - 1;
            let mut smudged = false;
            for idx in 0..min_len {
                if left[last - idx] == right[idx] {
                    continue;
                } else if !smudged && is_smudged(left[last - idx], right[idx]) {
                    smudged = true;
                    continue;
                } else {
                    return None;
                }
            }

            if smudged {
                Some(left.len() as u64)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0)
}

fn count_mirroed(line: &[u64]) -> u64 {
    (1..(line.len()))
        .map(|mid| line.split_at(mid))
        .filter_map(|(left, right)| {
            let min_len = left.len().min(right.len());
            let last = left.len() - 1;
            if (0..min_len).all(|idx| left[last - idx] == right[idx]) {
                Some(left.len() as u64)
            } else {
                None
            }
        })
        .max()
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
            let line_cnt = count_mirroed(&line);
            let column_cnt = count_mirroed(&column);
            line_cnt + column_cnt * 100
        })
        .sum()
}

fn part02(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|segment| (parse_line(segment), parse_column(segment)))
        .map(|(line, column)| {
            let line_cnt = count_smudged(&line);
            let column_cnt = count_smudged(&column);

            if line_cnt > 0 && column_cnt > 0 {
                if line_cnt > column_cnt {
                    line_cnt
                } else {
                    column_cnt * 100
                }
            } else {
                line_cnt + column_cnt * 100
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../input/day13.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn is_smudged() {
        assert!(super::is_smudged(305, 273));
        assert!(super::is_smudged(358, 102));
        assert!(!super::is_smudged(33, 33));
        assert!(!super::is_smudged(82, 33));
    }

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
    fn count_mirroed() {
        assert_eq!(
            super::count_mirroed(&[77, 12, 115, 33, 82, 82, 33, 115, 12]),
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

    #[test]
    fn part02() {
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

        assert_eq!(super::part02(input), 400);
    }
}

