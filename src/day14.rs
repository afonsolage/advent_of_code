fn calc_row_load(row: &[char]) -> u64 {
    let row_count = row.len();
    row.iter()
        .rev()
        .enumerate()
        .map(|(idx, &c)| if c == 'O' { (idx + 1) as u64 } else { 0 })
        .sum()
}

fn calc_platform_load(platform: &[Vec<char>]) -> u64 {
    platform.iter().map(|row| calc_row_load(row)).sum()
}

fn tilt_north(platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    platform
        .into_iter()
        .map(|mut column| {
            for n in 0..column.len() {
                if column[n] == '.' {
                    let next = n + column.iter().skip(n).take_while(|&&c| c == '.').count();
                    if next < column.len() && column[next] == 'O' {
                        column.swap(n, next);
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
    let row_count = input.lines().count();
    let column_count = input.lines().next().unwrap().len();

    input.lines().enumerate().fold(
        vec![vec!['.'; row_count]; column_count],
        |mut platform, (column, line)| {
            line.chars().enumerate().for_each(|(row, c)| {
                platform[row][column] = c;
            });
            platform
        },
    )
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
    let tilted_platform = tilt_north(platform);
    calc_platform_load(&tilted_platform)
}

fn part02(input: &str) -> u64 {
    let platform = parse(input);
    print_platform(&platform);
    let platform = transpose(platform);
    print_platform(&platform);
    let platform = transpose(platform);
    print_platform(&platform);
    let platform = transpose(platform);
    print_platform(&platform);
    64
}

fn main() {
    let input = include_str!("../input/day14.input");
    println!("Part 01: {}", part01(input));
    // println!("Part 02: {}", part02(input));
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

        assert_eq!(super::tilt_north(platform), tilted_platform);
    }

    #[test]
    fn calc_row_load() {
        assert_eq!(
            super::calc_row_load(&['O', 'O', 'O', 'O', '.', '.', '.', '.', '#', '#']),
            34
        );
    }

    #[test]
    fn part01() {
        assert_eq!(super::part01(INPUT), 136);
    }

    // #[test]
    // fn part02() {
    //     assert_eq!(super::part02(INPUT), 0);
    // }
}

