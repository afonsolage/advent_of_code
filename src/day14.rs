use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
};

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

fn tilt_row(row: &mut [char]) {}

fn tilt_north(platform: &mut [Vec<char>]) {
    platform.iter_mut().for_each(|column| {
        for n in 0..column.len() {
            if column[n] == '.' {
                let next = n + column.iter().skip(n).take_while(|&&c| c == '.').count();
                if next < column.len() && column[next] == 'O' {
                    column.swap(n, next);
                }
            }
        }
    });
}

fn tilt_west(platform: &mut [Vec<char>]) {
    let column_count = platform.len();
    let row_count = platform.first().unwrap().len();

    for r in 0..row_count {
        for c in 0..(column_count - 1) {
            if platform[c][r] == '.' {
                let mut next = c + 1;
                while next < (column_count - 1) && platform[next][r] == '.' {
                    next += 1;
                }

                if platform[next][r] == 'O' {
                    let tmp = platform[c][r];
                    platform[c][r] = platform[next][r];
                    platform[next][r] = tmp;
                }
            }
        }
    }
}

fn tilt_south(platform: &mut [Vec<char>]) {
    let column_count = platform.len();
    let row_count = platform.first().unwrap().len();

    platform.iter_mut().for_each(|column| {
        for r in (1..row_count).rev() {
            if column[r] == '.' {
                let mut prev = r - 1;
                while prev > 0 && column[prev] == '.' {
                    prev -= 1;
                }

                if column[prev] == 'O' {
                    column.swap(prev, r);
                }
            }
        }
    });
}

fn tilt_east(platform: &mut [Vec<char>]) {
    let column_count = platform.len();
    let row_count = platform.first().unwrap().len();

    for r in 0..row_count {
        for c in (1..column_count).rev() {
            if platform[c][r] == '.' {
                let mut prev = c - 1;
                while prev > 0 && platform[prev][r] == '.' {
                    prev -= 1;
                }

                if platform[prev][r] == 'O' {
                    let tmp = platform[c][r];
                    platform[c][r] = platform[prev][r];
                    platform[prev][r] = tmp;
                }
            }
        }
    }
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

fn print(platform: &[Vec<char>]) {
    let row_count = platform.first().unwrap().len();
    let column_count = platform.len();
    for c in 0..column_count {
        for row in platform {
            print!("{}", row[c]);
        }
        println!();
    }
    println!();
}

fn hash(platform: &[Vec<char>]) -> u64 {
    let mut hasher = DefaultHasher::new();

    platform.hash(&mut hasher);

    hasher.finish()
}

fn part01(input: &str) -> u64 {
    let mut platform = parse(input);
    tilt_north(&mut platform);
    calc_platform_load(&platform)
}

struct CacheablePlatform(u64, Vec<Vec<char>>);

fn cycle(cache: &mut HashMap<u64, Vec<Vec<char>>>, platform: &mut CacheablePlatform) {
    if let Some(cached) = cache.get(&platform.0) {
        platform
            .1
            .iter_mut()
            .zip(cached)
            .for_each(|(column, cached_column)| column.copy_from_slice(cached_column));
    } else {
        tilt_north(&mut platform.1);
        tilt_west(&mut platform.1);
        tilt_south(&mut platform.1);
        tilt_east(&mut platform.1);
        platform.0 = hash(&platform.1);
    }
}

fn part02(input: &str) -> u64 {
    let platform = parse(input);
    let hash = hash(&platform);

    let mut platform = CacheablePlatform(hash, platform);
    let mut cache = HashMap::new();

    for n in 0..1000 {
        cycle(&mut cache, &mut platform);
    }

    calc_platform_load(&platform.1)
}

fn main() {
    let input = include_str!("../input/day14.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

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
        let mut platform = super::parse(INPUT);
        let tilted_platform = super::parse(TILTED_INPUT);

        super::tilt_north(&mut platform);

        assert_eq!(platform, tilted_platform);
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

    #[test]
    fn part02() {
        assert_eq!(super::part02(INPUT), 64);
    }
}

