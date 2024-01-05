#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn from_iter(mut it: impl Iterator<Item = i32>) -> Self {
        Self {
            x: it.next().unwrap(),
            y: it.next().unwrap(),
            z: it.next().unwrap(),
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Brick {
    id: u32,
    begin: Vec3,
    end: Vec3,
}

impl Brick {
    fn move_down(mut self) -> Self {
        self.begin.z -= 1;
        self.end.z -= 1;
        self
    }

    fn intersects(&self, brick: Brick) -> bool {
        self.begin.x <= brick.end.x
            && self.end.x >= brick.begin.x
            && self.begin.y <= brick.end.y
            && self.end.y >= brick.begin.y
            && self.begin.z <= brick.end.z
            && self.end.z >= brick.begin.z
    }
}

impl std::fmt::Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}~{}", self.id, self.begin, self.end)
    }
}

fn can_move_down(bricks: &[Brick], brick: Brick) -> bool {
    let brick = brick.move_down();
    brick.end.z > 0
        && !bricks
            .iter()
            .filter(|b| b.id != brick.id)
            .any(|b| b.intersects(brick))
}

fn settle_down(bricks: &mut [Brick]) {
    for i in 0..bricks.len() {
        let mut brick = bricks[i];
        println!("Working on brick {brick:?}");

        if brick.end.z == 1 {
            continue;
        }

        while can_move_down(bricks, brick) {
            let after = brick.move_down();
            println!("Moving {brick} to {after}");
            brick = brick.move_down();
        }
        bricks[i] = brick;
    }
}

fn print_y_z(bricks: &[Brick]) {
    let (width, height) = bricks.iter().fold((0, 0), |(width, height), brick| {
        let width = width.max(brick.end.y);
        let height = height.max(brick.end.z);

        (width, height)
    });
    println!("Z");
    for _ in 0..width + 3 {
        print!("_");
    }
    println!();
    for z in (1..height + 1).rev() {
        print!("|");
        for y in 0..(width + 1) {
            let p = Vec3 { x: 1, y, z };
            if let Some(brick) = bricks
                .iter()
                .find(|b| b.begin.y <= p.y && b.end.y >= p.y && b.begin.z <= p.z && b.end.z >= p.z)
            {
                print!("{}", brick.id);
            } else {
                print!(" ");
            }
        }
        print!("|");
        println!();
    }

    for _ in 0..width + 3 {
        print!("¯");
    }
    println!(" Y");
}

fn print_x_z(bricks: &[Brick]) {
    let (width, height) = bricks.iter().fold((0, 0), |(width, height), brick| {
        let width = width.max(brick.end.x);
        let height = height.max(brick.end.z);

        (width, height)
    });
    println!("Z");
    for _ in 0..width + 3 {
        print!("_");
    }
    println!();
    for z in (1..height + 1).rev() {
        print!("|");
        for x in 0..(width + 1) {
            let p = Vec3 { x, y: 1, z };
            if let Some(brick) = bricks
                .iter()
                .find(|b| b.begin.x <= p.x && b.end.x >= p.x && b.begin.z <= p.z && b.end.z >= p.z)
            {
                print!("{}", brick.id);
            } else {
                print!(" ");
            }
        }
        print!("|");
        println!();
    }
    for _ in 0..width + 3 {
        print!("¯");
    }
    println!(" X");
}

fn print_bricks(bricks: &[Brick]) {
    print_x_z(bricks);
    print_y_z(bricks);
}

fn part01(input: &str) -> u64 {
    let mut next_id = 0;
    let mut bricks = input
        .lines()
        .map(|line| {
            line.split_once('~')
                .map(|(begin, end)| {
                    let begin =
                        Vec3::from_iter(begin.split(',').map(|n| n.parse::<i32>().unwrap()));
                    let end = Vec3::from_iter(end.split(',').map(|n| n.parse::<i32>().unwrap()));
                    let id = next_id;
                    next_id += 1;
                    Brick { begin, end, id }
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    bricks.sort_unstable_by(|a, b| a.begin.z.cmp(&b.begin.z));

    print_bricks(&bricks);

    settle_down(&mut bricks);

    print_bricks(&bricks);

    0
}

fn part02(_input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input/day22.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {
    // Tests if two bricks (BoundingBoxes) intersects each other
    // Two bricks intersects if they AABB represented by begin and end, intersects
    #[test]
    fn brick_intersects() {
        let brick = super::Brick {
            id: 0,
            begin: super::Vec3 { x: 0, y: 0, z: 0 },
            end: super::Vec3 { x: 1, y: 1, z: 1 },
        };

        assert!(brick.intersects(brick));

        assert!(brick.intersects(super::Brick {
            id: 1,
            begin: super::Vec3 { x: 0, y: 0, z: 0 },
            end: super::Vec3 { x: 1, y: 1, z: 1 },
        }));

        assert!(brick.intersects(super::Brick {
            id: 2,
            begin: super::Vec3 { x: 0, y: 0, z: 0 },
            end: super::Vec3 { x: 1, y: 1, z: 2 },
        }));

        assert!(brick.intersects(super::Brick {
            id: 3,
            begin: super::Vec3 { x: 0, y: 0, z: 0 },
            end: super::Vec3 { x: 1, y: 2, z: 1 },
        }));

        assert!(brick.intersects(super::Brick {
            id: 4,
            begin: super::Vec3 { x: 0, y: 0, z: 0 },
            end: super::Vec3 { x: 2, y: 1, z: 1 },
        }));

        assert!(brick.intersects(super::Brick {
            id: 5,
            begin: super::Vec3 { x: 0, y: 0, z: 0 },
            end: super::Vec3 { x: 2, y: 2, z: 2 },
        }));

        assert!(brick.intersects(super::Brick {
            id: 6,
            begin: super::Vec3 { x: 0, y: 0, z: 1 },
            end: super::Vec3 { x: 1, y: 1, z: 2 },
        }));
    }

    #[test]
    fn part01() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        assert_eq!(super::part01(input), 5);
    }

    #[test]
    fn part02() {
        let input = "";

        assert_eq!(super::part02(input), 0);
    }
}
