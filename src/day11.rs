use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum MapCell {
    Galaxy,
    Void(i32, i32),
}

impl MapCell {
    fn distance(&self) -> (i32, i32) {
        match *self {
            MapCell::Galaxy => (1, 1),
            MapCell::Void(w, h) => (w, h),
        }
    }
}

fn expand_galaxy(input: &str, distance: i32) -> Vec<Vec<MapCell>> {
    let mut map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '#' {
                        MapCell::Galaxy
                    } else {
                        MapCell::Void(1, 1)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for row in map.iter_mut() {
        if row.iter_mut().all(|c| matches!(c, MapCell::Void(..))) {
            row.iter_mut().for_each(|c| {
                if let MapCell::Void(_, ref mut h) = c {
                    *h = distance;
                }
            });
        }
    }

    let width = map.first().unwrap().len();
    for i in 0..width {
        if map
            .iter()
            .all(|row| matches!(row.get(i).unwrap(), MapCell::Void(..)))
        {
            map.iter_mut().for_each(|row| {
                if let MapCell::Void(ref mut w, _) = row.get_mut(i).unwrap() {
                    *w = distance;
                }
            })
        }
    }

    map
}

fn calc_distance(source: (i32, i32), dest: (i32, i32), distance: (i32, i32)) -> i32 {
    let x_offset = (dest.0 - source.0).abs() * distance.0;
    let y_offset = (dest.1 - source.1).abs() * distance.1;
    x_offset + y_offset
}

fn calc_path_distance(map: &[Vec<MapCell>], path: &[(i32, i32)]) -> u64 {
    path.windows(2)
        .map(|pair| {
            let a = pair[0];
            let b = pair[1];

            let distance_b_a = map[b.1 as usize][b.0 as usize].distance();
            calc_distance(a, b, distance_b_a) as u64
        })
        .sum()
}

fn parse_galaxy_map(input: &str, galaxy_distance: i32) -> u64 {
    let map = expand_galaxy(input, galaxy_distance);

    let galaxies = map
        .iter()
        .enumerate()
        .flat_map(|(y, chars)| {
            chars.iter().enumerate().filter_map(move |(x, c)| {
                if *c == MapCell::Galaxy {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(idx, source)| {
            galaxies
                .iter()
                .skip(idx + 1)
                .map(|&dest| find_shortest_path(&map, &mut HashSet::new(), *source, dest))
        })
        .map(|path| calc_path_distance(&map, &path))
        .sum()
}

fn part01(input: &str) -> u64 {
    parse_galaxy_map(input, 2)
}

fn part02(input: &str) -> u64 {
    parse_galaxy_map(input, 1_000_000)
}

fn find_shortest_path(
    map: &[Vec<MapCell>],
    visited: &mut HashSet<(i32, i32)>,
    source: (i32, i32),
    dest: (i32, i32),
) -> Vec<(i32, i32)> {
    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    if source == dest {
        return vec![source];
    }

    visited.insert(source);

    let mut nexts = DIRECTIONS
        .iter()
        .map(|dir| (source.0 + dir.0, source.1 + dir.1))
        .filter(|source| {
            source.0 >= 0
                && (source.0 as usize) < map[0].len()
                && source.1 >= 0
                && (source.1 as usize) < map.len()
                && !visited.contains(source)
        })
        .collect::<Vec<_>>();

    nexts.sort_unstable_by(|&a, &b| {
        calc_distance(a, dest, (1, 1)).cmp(&calc_distance(b, dest, (1, 1)))
    });

    for next in nexts {
        let mut next_path = find_shortest_path(map, visited, next, dest);
        if !next_path.is_empty() {
            next_path.insert(0, source);
            return next_path;
        }
    }

    vec![]
}

fn main() {
    let input = include_str!("../input/day11.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    const GALAXIES: [(i32, i32); 9] = [
        (3, 0),
        (7, 1),
        (0, 2),
        (6, 4),
        (1, 5),
        (9, 6),
        (7, 8),
        (0, 9),
        (4, 9),
    ];

    #[test]
    fn part01() {
        assert_eq!(super::part01(INPUT), 374);
    }

    #[test]
    fn part01_path_5_9() {
        let grid = expand_galaxy(INPUT, 2);

        let path = find_shortest_path(&grid, &mut HashSet::new(), GALAXIES[4], GALAXIES[8]);
        let distance = calc_path_distance(&grid, &path);

        assert_eq!(distance, 9);
    }

    #[test]
    fn part01_path_1_7() {
        let grid = expand_galaxy(INPUT, 2);

        let path = find_shortest_path(&grid, &mut HashSet::new(), GALAXIES[0], GALAXIES[6]);
        let distance = calc_path_distance(&grid, &path);

        assert_eq!(distance, 15);
    }

    #[test]
    fn part01_path_3_6() {
        let grid = expand_galaxy(INPUT, 2);

        let path = find_shortest_path(&grid, &mut HashSet::new(), GALAXIES[2], GALAXIES[5]);
        let distance = calc_path_distance(&grid, &path);

        assert_eq!(distance, 17);
    }

    #[test]
    fn part01_path_8_9() {
        let grid = expand_galaxy(INPUT, 2);

        let path = find_shortest_path(&grid, &mut HashSet::new(), GALAXIES[7], GALAXIES[8]);
        let distance = calc_path_distance(&grid, &path);

        assert_eq!(distance, 5);
    }

    #[test]
    fn part01_path_7_9() {
        let grid = expand_galaxy(INPUT, 2);

        let path = find_shortest_path(&grid, &mut HashSet::new(), GALAXIES[6], GALAXIES[8]);
        let distance = calc_path_distance(&grid, &path);

        assert_eq!(distance, 5);
    }

    #[test]
    fn part01_path_1_4() {
        let grid = expand_galaxy(INPUT, 2);

        let path = find_shortest_path(&grid, &mut HashSet::new(), GALAXIES[0], GALAXIES[3]);
        let distance = calc_path_distance(&grid, &path);

        assert_eq!(distance, 9);
    }

    #[test]
    fn part02_10() {
        assert_eq!(super::parse_galaxy_map(INPUT, 10), 1030);
    }

    #[test]
    fn part02_100() {
        assert_eq!(super::parse_galaxy_map(INPUT, 100), 8410);
    }
}

