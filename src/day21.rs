use std::collections::{HashSet, VecDeque};

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn walk_to_plots(map: &[Vec<char>], start: (i32, i32), steps: u32) -> u64 {
    let width = map[0].len();
    let height = map.len();

    let mut queue = VecDeque::new();
    queue.push_back((start, steps));

    let mut finished = HashSet::new();
    let mut seen = HashSet::new();

    while let Some((pos, steps_left)) = queue.pop_front() {
        if pos.0 < 0 || pos.1 < 0 || pos.0 as usize >= width || pos.1 as usize >= height {
            continue;
        }

        if map[pos.1 as usize][pos.0 as usize] == '#' {
            continue;
        }

        if steps_left == 0 {
            finished.insert(pos);
            continue;
        }

        if seen.contains(&(pos, steps_left)) {
            continue;
        }

        seen.insert((pos, steps_left));

        for dir in DIRS {
            let next_dir = (pos.0 + dir.0, pos.1 + dir.1);

            queue.push_back((next_dir, steps_left - 1));
        }
    }

    finished.len() as u64
}

fn part01(input: &str, steps: u32) -> u64 {
    let mut start = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x as i32, y as i32);
                        '.'
                    } else {
                        c
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    walk_to_plots(&map, start, steps)
}

fn part02(_input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input/day21.input");
    println!("Part 01: {}", part01(input, 64));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        assert_eq!(super::part01(input, 6), 16);
    }

    #[test]
    fn part02() {
        let input = "";

        assert_eq!(super::part02(input), 0);
    }
}

