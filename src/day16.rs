#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Top,
    Right,
    Bottom,
    Left,
}

impl Dir {
    fn advance(&self, position: (i32, i32)) -> (i32, i32) {
        match self {
            Dir::Top => (position.0, position.1 - 1),
            Dir::Right => (position.0 + 1, position.1),
            Dir::Bottom => (position.0, position.1 + 1),
            Dir::Left => (position.0 - 1, position.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    // .
    Empty,
    // /
    MirrorLr,
    // \
    MirrorRl,
    // -
    SplitterH,
    // |
    SplitterV,
}

impl Cell {
    fn bounce(&self, dir: Dir) -> (Dir, Option<Dir>) {
        match self {
            Cell::Empty => (dir, None),
            Cell::MirrorLr => match dir {
                Dir::Top => (Dir::Right, None),
                Dir::Right => (Dir::Top, None),
                Dir::Bottom => (Dir::Left, None),
                Dir::Left => (Dir::Bottom, None),
            },
            Cell::MirrorRl => match dir {
                Dir::Top => (Dir::Left, None),
                Dir::Right => (Dir::Bottom, None),
                Dir::Bottom => (Dir::Right, None),
                Dir::Left => (Dir::Top, None),
            },
            Cell::SplitterH => match dir {
                Dir::Top | Dir::Bottom => (Dir::Right, Some(Dir::Left)),
                Dir::Right | Dir::Left => (dir, None),
            },
            Cell::SplitterV => match dir {
                Dir::Top | Dir::Bottom => (dir, None),
                Dir::Right | Dir::Left => (Dir::Top, Some(Dir::Bottom)),
            },
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::MirrorLr,
            '\\' => Self::MirrorRl,
            '-' => Self::SplitterH,
            '|' => Self::SplitterV,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::MirrorLr => write!(f, "/"),
            Cell::MirrorRl => write!(f, "\\"),
            Cell::SplitterH => write!(f, "-"),
            Cell::SplitterV => write!(f, "|"),
        }
    }
}

#[derive(Debug, Clone)]
struct Contraption {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Contraption {
    fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let cells = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    fn fire_beam(&self, pos: (i32, i32), dir: Dir, path: &mut Vec<(i32, i32)>) {
        println!("Firing beam {dir:?} at {pos:?}");
        if pos.0 < 0 || pos.1 < 0 || pos.0 as usize >= self.width || pos.1 as usize >= self.height {
            return;
        }

        path.push(pos);

        let cell = self.cells[pos.1 as usize][pos.0 as usize];
        let (next_dir, maybe_splitted_next_dir) = cell.bounce(dir);

        let next_pos = next_dir.advance(pos);
        self.fire_beam(next_pos, next_dir, path);

        if let Some(next_dir) = maybe_splitted_next_dir {
            let next_pos = next_dir.advance(pos);
            self.fire_beam(next_pos, next_dir, path);
        }
    }
}

fn part01(input: &str) -> u64 {
    let contraption = Contraption::new(input);
    let mut beam_path = vec![];
    contraption.fire_beam((0, 0), Dir::Right, &mut beam_path);

    let mut heat_map = vec![vec![0; contraption.width]; contraption.height];

    for pos in beam_path {
        heat_map[pos.1 as usize][pos.0 as usize] += 1;
    }

    heat_map.iter().for_each(|line| {
        line.iter().for_each(|cell| print!("{cell} "));
        println!();
    });

    heat_map.into_iter().flatten().filter(|&c| c == 1).count() as u64
}

fn part02(_input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input/day16.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        assert_eq!(super::part01(input), 46);
    }

    #[test]
    fn part02() {
        let input = "";

        assert_eq!(super::part02(input), 0);
    }
}
