fn calc_row_load(row: &[char]) -> u64 {
    row.iter()
        .enumerate()
        .map(|(idx, &c)| if c == 'O' { (idx + 1) as u64 } else { 0 })
        .sum()
}

fn calc_platform_load(platform: &[Vec<char>]) -> u64 {
    platform.iter().map(|row| calc_row_load(row)).sum()
}

fn tilt_right(platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    platform
        .into_iter()
        .map(|mut column| {
            for n in (0..column.len()).rev() {
                if column[n] == '.' {
                    let move_left = column
                        .iter()
                        .take(n + 1)
                        .rev()
                        .take_while(|&&c| c == '.')
                        .count();
                    if move_left > 0 && move_left <= n && column[n - move_left] == 'O' {
                        column.swap(n, n - move_left);
                    }
                }
            }
            column
        })
        .collect()
}

fn transpose(platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row_count = platform.len();
    let column_count = platform.first().unwrap().len();

    platform.into_iter().enumerate().fold(
        vec![vec!['.'; row_count]; column_count],
        |mut transposed, (r_idx, row)| {
            row.into_iter().enumerate().for_each(|(c_idx, value)| {
                transposed[c_idx][row_count - 1 - r_idx] = value;
            });

            transposed
        },
    )
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn print_platform(platform: &[Vec<char>]) {
    platform.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    });

    println!();
}

fn part01(input: &str) -> u64 {
    let platform = parse(input);
    let platform = transpose(platform);
    let tilted_platform = tilt_right(platform);
    calc_platform_load(&tilted_platform)
}

fn part02(_input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input/day14.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {
    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const TILTED_INPUT: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    #[test]
    fn tilt_north() {
        let platform = super::parse(INPUT);
        let tilted_platform = super::parse(TILTED_INPUT);

        let transposed_plat = super::transpose(platform);
        let transposed_tilted_plat = super::transpose(tilted_platform);

        assert_eq!(super::tilt_right(transposed_plat), transposed_tilted_plat);
    }

    #[test]
    fn calc_row_load() {
        assert_eq!(
            super::calc_row_load(&['#', '#', '.', '.', '.', '.', 'O', 'O', 'O', 'O']),
            34
        );
    }

    #[test]
    fn part01() {
        assert_eq!(super::part01(INPUT), 136);
    }

    #[test]
    fn part02() {
        assert_eq!(super::part02(INPUT), 0);
    }
}

