use std::collections::VecDeque;

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
struct BoundingBox {
    begin: Vec3,
    end: Vec3,
}

impl std::fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}~{}", self.begin, self.end)
    }
}

impl BoundingBox {
    fn move_down(mut self) -> Self {
        self.begin.z -= 1;
        self.end.z -= 1;
        self
    }

    fn move_up(mut self) -> Self {
        self.begin.z += 1;
        self.end.z += 1;
        self
    }

    fn intersects(&self, other: &Self) -> bool {
        self.begin.x <= other.end.x
            && self.end.x >= other.begin.x
            && self.begin.y <= other.end.y
            && self.end.y >= other.begin.y
            && self.begin.z <= other.end.z
            && self.end.z >= other.begin.z
    }
}

#[derive(Default, Debug, Clone)]
struct Brick {
    id: usize,
    bb: BoundingBox,
    above: Vec<usize>,
    below: Vec<usize>,
}

impl Brick {
    fn letter(&self) -> char {
        ((self.id as u8 % 25) + 65).try_into().unwrap()
    }
}

impl std::fmt::Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}][{}] {}", self.letter(), self.id, self.bb)
    }
}

fn settle_down(bricks: &mut [Brick]) {
    for i in 0..bricks.len() {
        while bricks[i].bb.begin.z > 1 {
            let below = bricks[i].bb.move_down();

            if bricks
                .iter()
                .filter(|b| b.id != i)
                .any(|b| b.bb.intersects(&below))
            {
                break;
            } else {
                bricks[i].bb = below;
            }
        }
    }

    set_above_and_below(bricks);
}

fn set_above_and_below(bricks: &mut [Brick]) {
    for i in 0..bricks.len() {
        let below = bricks[i].bb.move_down();
        bricks[i].below = bricks
            .iter()
            .filter_map(|b| {
                if b.id != i && below.intersects(&b.bb) {
                    Some(b.id)
                } else {
                    None
                }
            })
            .collect();

        let above = bricks[i].bb.move_up();
        bricks[i].above = bricks
            .iter()
            .filter_map(|b| {
                if b.id != i && above.intersects(&b.bb) {
                    Some(b.id)
                } else {
                    None
                }
            })
            .collect();
    }
}

fn can_remove(bricks: &[Brick], brick: &Brick) -> bool {
    bricks
        .iter()
        .filter(|b| brick.above.contains(&b.id))
        .all(|b| b.below.len() > 1)
}

fn chain_reaction(bricks: &[Brick]) -> usize {
    let mut queue = VecDeque::new();
    let mut falling = vec![false; bricks.len()];
    let mut count = 0;

    for b in bricks {
        queue.push_back(b.id);
        falling[b.id] = true;

        while let Some(id) = queue.pop_front() {
            for &upper_id in &bricks[id].above {
                if !falling[upper_id] && will_fall(&bricks[upper_id], &falling) {
                    queue.push_back(upper_id);
                    falling[upper_id] = true;
                    count += 1;
                }
            }
        }
        falling.fill(false);
    }

    count
}

fn will_fall(brick: &Brick, falling: &[bool]) -> bool {
    brick.below.iter().all(|&id| falling[id])
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    let mut bricks = input
        .lines()
        .map(|line| {
            line.split_once('~')
                .map(|(begin, end)| {
                    let mut begin =
                        Vec3::from_iter(begin.split(',').map(|n| n.parse::<i32>().unwrap()));
                    let mut end =
                        Vec3::from_iter(end.split(',').map(|n| n.parse::<i32>().unwrap()));

                    if begin.x > end.x {
                        std::mem::swap(&mut begin.x, &mut end.x);
                    }
                    if begin.y > end.y {
                        std::mem::swap(&mut begin.y, &mut end.y);
                    }
                    if begin.z > end.z {
                        std::mem::swap(&mut begin.z, &mut end.z);
                    }

                    let bb = BoundingBox { begin, end };
                    Brick {
                        bb,
                        ..Default::default()
                    }
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    bricks.sort_unstable_by_key(|b| b.bb.end.z);
    bricks.iter_mut().enumerate().for_each(|(id, b)| b.id = id);

    bricks
}

fn part01(input: &str) -> u64 {
    let mut bricks = parse_bricks(input);
    settle_down(&mut bricks);

    bricks.iter().filter(|&b| can_remove(&bricks, b)).count() as u64
}

fn part02(input: &str) -> u64 {
    let mut bricks = parse_bricks(input);
    settle_down(&mut bricks);

    chain_reaction(&bricks) as u64
}

fn main() {
    let input = include_str!("../input/day22.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {
    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn part01() {
        assert_eq!(super::part01(INPUT), 5);
    }

    #[test]
    fn part02() {
        assert_eq!(super::part02(INPUT), 7);
    }
}
