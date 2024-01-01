#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn splat(n: i32) -> Self {
        Self { x: n, y: n }
    }

    fn from_dir(input: &str) -> Self {
        match input {
            "U" => Self { x: 0, y: -1 },
            "R" => Self { x: 1, y: 0 },
            "D" => Self { x: 0, y: 1 },
            "L" => Self { x: -1, y: 0 },
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.y,
            y: self.y * rhs.y,
        }
    }
}

fn calculate_area(points: &[Vec2]) -> i32 {
    points.iter().enumerate().fold(0, |acc, (idx, p)| {
        let next = (idx + 1) % points.len();
        let n = points[next];
        acc + (p.x + n.x) * (n.y - p.y)
    }) / 2
}

fn part01(input: &str) -> u64 {
    let corners = [
        Vec2 { x: 1, y: -1 },
        Vec2::splat(1),
        Vec2 { x: -1, y: 1 },
        Vec2::splat(-1),
    ];

    let offsets = [
        Vec2::splat(0),
        Vec2 { x: 1, y: 0 },
        Vec2::splat(1),
        Vec2 { x: 0, y: 1 },
    ];

    let dig_plan = input
        .lines()
        .map(|line| {
            let mut it = line.split(|c: char| c.is_ascii_whitespace());
            let dir = it.next().unwrap();
            let meters = it.next().unwrap();
            let color = it.next().unwrap().trim_matches(|c| c == '(' || c == ')');
            (dir, meters, color)
        })
        .fold(
            vec![(Vec2::splat(0), Vec2 { x: 0, y: -1 })],
            |mut points, (dir, meters, _color)| {
                let (previous, _) = *points.last().unwrap();
                let meters = Vec2::splat(meters.parse().unwrap());
                let dir = Vec2::from_dir(dir);
                let point = previous + dir * meters;

                points.push((point, dir));
                points
            },
        )
        .windows(2)
        .map(|w| {
            let (point, dir) = w[0];
            let (_, next_dir) = w[1];

            let corner = dir + next_dir;
            let offset = offsets[corners.iter().position(|&c| c == corner).unwrap()];
            point + offset
        })
        .collect::<Vec<_>>();

    calculate_area(&dig_plan) as u64
}

fn part02(_input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input/day18.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {
    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part01() {
        assert_eq!(super::part01(INPUT), 62);
    }

    #[test]
    fn part02() {
        let input = "";

        assert_eq!(super::part02(input), 0);
    }
}

