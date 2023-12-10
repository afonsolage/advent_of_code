use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Groud,
    Starting,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Vertical => write!(f, "|"),
            Tile::Horizontal => write!(f, "-"),
            Tile::NorthEast => write!(f, "L"),
            Tile::NorthWest => write!(f, "J"),
            Tile::SouthWest => write!(f, "7"),
            Tile::SouthEast => write!(f, "F"),
            Tile::Groud => write!(f, "."),
            Tile::Starting => write!(f, "S"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn from_index(width: isize, index: isize) -> Self {
        Self {
            y: index / width,
            x: index % width,
        }
    }

    fn to_index(&self, width: isize) -> isize {
        self.y * width + self.x
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Tile {
    fn connections(&self) -> [Pos; 2] {
        match self {
            Tile::Vertical => [Pos { x: 0, y: -1 }, Pos { x: 0, y: 1 }],
            Tile::Horizontal => [Pos { x: -1, y: 0 }, Pos { x: 1, y: 0 }],
            Tile::NorthEast => [Pos { x: 0, y: -1 }, Pos { x: 1, y: 0 }],
            Tile::NorthWest => [Pos { x: 0, y: -1 }, Pos { x: -1, y: 0 }],
            Tile::SouthWest => [Pos { x: 0, y: 1 }, Pos { x: -1, y: 0 }],
            Tile::SouthEast => [Pos { x: 0, y: 1 }, Pos { x: 1, y: 0 }],
            _ => unreachable!(),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Groud,
            'S' => Self::Starting,
            _ => unreachable!(),
        }
    }
}

fn part01(input: &str) -> u64 {
    let map = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.into()))
        .collect::<Vec<Tile>>();

    let width = (map.len() as f64).sqrt() as isize;
    let start = map.iter().position(|&t| t == Tile::Starting).unwrap();
    let start_pos = Pos::from_index(width, start as isize);

    let next = |previous: Pos, current: Pos| -> Pos {
        let pipe = map[current.to_index(width) as usize];
        let [mut a, mut b] = pipe.connections();

        a = a + current;
        b = b + current;

        if previous == a {
            b
        } else {
            a
        }
    };

    let (mut a, mut b) = { calc_starting_connections(&map, width, start_pos) };

    let mut previous_a = start_pos;
    let mut previous_b = start_pos;
    let mut depth = 1;

    loop {
        depth += 1;
        let next_a = next(previous_a, a);
        let next_b = next(previous_b, b);

        if next_a == next_b {
            break depth;
        }

        previous_a = a;
        previous_b = b;
        a = next_a;
        b = next_b;
    }
}

fn calc_starting_connections(map: &Vec<Tile>, width: isize, start_pos: Pos) -> (Pos, Pos) {
    let top = Pos { x: 0, y: -1 } + start_pos;
    let right = Pos { x: 1, y: 0 } + start_pos;
    let bottom = Pos { x: 0, y: 1 } + start_pos;
    let left = Pos { x: -1, y: 0 } + start_pos;

    let mut result = vec![];

    if top.y >= 0 {
        let top_tile = map[top.to_index(width) as usize];

        if matches!(top_tile, Tile::Vertical | Tile::SouthEast | Tile::SouthWest) {
            result.push(top);
        }
    }

    if bottom.y < width {
        let bottom_tile = map[bottom.to_index(width) as usize];

        if matches!(
            bottom_tile,
            Tile::Vertical | Tile::NorthWest | Tile::NorthEast
        ) {
            result.push(bottom);
        }
    }

    if left.x >= 0 {
        let left_tile = map[left.to_index(width) as usize];

        if matches!(
            left_tile,
            Tile::Horizontal | Tile::SouthEast | Tile::NorthEast
        ) {
            result.push(left);
        }
    }

    if right.x < width {
        let right_tile = map[right.to_index(width) as usize];

        if matches!(
            right_tile,
            Tile::Horizontal | Tile::SouthWest | Tile::NorthWest
        ) {
            result.push(right);
        }
    }

    (result[0], result[1])
}

fn part02(input: &str) -> u64 {
    let map = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.into()))
        .collect::<Vec<Tile>>();

    let width = (map.len() as f64).sqrt() as isize;
    let start = map.iter().position(|&t| t == Tile::Starting).unwrap();
    let start_pos = Pos::from_index(width, start as isize);

    let next = |previous: Pos, current: Pos| -> Pos {
        let pipe = map[current.to_index(width) as usize];
        let [mut a, mut b] = pipe.connections();

        a = a + current;
        b = b + current;

        if previous == a {
            b
        } else {
            a
        }
    };

    let (mut a, mut b) = { calc_starting_connections(&map, width, start_pos) };

    let mut previous_a = start_pos;
    let mut previous_b = start_pos;
    let mut loop_map = HashSet::new();

    loop {
        let next_a = next(previous_a, a);
        let next_b = next(previous_b, b);

        if next_a == next_b {
            loop_map.insert(next_a);
            break;
        }

        previous_a = a;
        previous_b = b;
        a = next_a;
        b = next_b;
    }

    let mut inside_count = 0;

    for x in 0..width {
        for y in 0..width {
            let pos = Pos { x, y };

            if loop_map.contains(&pos) {
                continue;
            }

            if count_intersections(width, Pos { x: -1, y: 0 }, pos, &loop_map) % 2 == 0 {
                continue;
            }
            if count_intersections(width, Pos { x: 1, y: 0 }, pos, &loop_map) % 2 == 0 {
                continue;
            }
            if count_intersections(width, Pos { x: 0, y: 1 }, pos, &loop_map) % 2 == 0 {
                continue;
            }
            if count_intersections(width, Pos { x: 0, y: -1 }, pos, &loop_map) % 2 == 0 {
                continue;
            }

            inside_count += 1;
        }
    }

    inside_count
}

fn count_intersections(width: isize, dir: Pos, position: Pos, map: &HashSet<Pos>) -> usize {
    let mut intersection_count = 0;
    let mut next_pos = position + dir;

    while next_pos.x >= 0 && next_pos.y >= 0 && next_pos.x < width && next_pos.y < width {
        if map.contains(&next_pos) {
            intersection_count += 1;
        }

        next_pos = next_pos + dir;
    }

    intersection_count
}

fn main() {
    let input = include_str!("../input/day10.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        assert_eq!(super::part01(input), 4);
    }

    #[test]
    fn part01_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        assert_eq!(super::part01(input), 8);
    }

    #[test]
    fn part02_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        assert_eq!(super::part02(input), 4);
    }

    #[test]
    fn part02_2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(super::part02(input), 8);
    }

    #[test]
    fn part02_3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        assert_eq!(super::part02(input), 10);
    }
}
