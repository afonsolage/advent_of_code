use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Eq)]
struct Crucible {
    heat_loss: u32,
    pos: (i32, i32),
    dir: Dir,
    consecultive_dir: u32,
}

impl Crucible {
    fn get_successors(&self, map: &[Vec<u32>]) -> [Option<Self>; 3] {
        let width = map[0].len();
        let height = map.len();

        let mut successors = [None, None, None];
        let mut idx = 0;

        for dir in Dir::all() {
            if self.dir == dir && self.consecultive_dir == 3 {
                continue;
            }

            if self.dir.inverse() == dir {
                continue;
            }

            let next_pos = (self.pos.0 + dir.x(), self.pos.1 + dir.y());

            if next_pos.0 < 0
                || next_pos.1 < 0
                || next_pos.0 as usize >= width
                || next_pos.1 as usize >= height
            {
                continue;
            }

            let next_heat_loss = self.heat_loss + map[next_pos.1 as usize][next_pos.0 as usize];
            let next_consecultive_dir = if self.dir == dir {
                self.consecultive_dir + 1
            } else {
                1
            };

            successors[idx] = Some(Crucible {
                heat_loss: next_heat_loss,
                pos: next_pos,
                dir,
                consecultive_dir: next_consecultive_dir,
            });
            idx += 1;
        }

        successors
    }

    fn get_super_successors(&self, map: &[Vec<u32>]) -> [Option<Crucible>; 3] {
        let width = map[0].len();
        let height = map.len();

        let mut successors = [None, None, None];
        let mut idx = 0;

        for dir in Dir::all() {
            if self.consecultive_dir < 4 && dir != self.dir {
                continue;
            }

            if self.consecultive_dir >= 10 && dir == self.dir {
                continue;
            }

            if self.dir.inverse() == dir {
                continue;
            }

            let next_pos = (self.pos.0 + dir.x(), self.pos.1 + dir.y());

            if next_pos.0 < 0
                || next_pos.1 < 0
                || next_pos.0 as usize >= width
                || next_pos.1 as usize >= height
            {
                continue;
            }

            let next_heat_loss = self.heat_loss + map[next_pos.1 as usize][next_pos.0 as usize];
            let next_consecultive_dir = if self.dir == dir {
                self.consecultive_dir + 1
            } else {
                1
            };

            successors[idx] = Some(Crucible {
                heat_loss: next_heat_loss,
                pos: next_pos,
                dir,
                consecultive_dir: next_consecultive_dir,
            });
            idx += 1;
        }

        successors
    }
}

impl std::cmp::PartialEq for Crucible {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss == other.heat_loss && self.pos == other.pos
    }
}

impl std::cmp::Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl std::cmp::PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Top,
    Right,
    Bottom,
    Left,
}

impl Dir {
    fn x(&self) -> i32 {
        match self {
            Dir::Right => 1,
            Dir::Left => -1,
            _ => 0,
        }
    }

    fn y(&self) -> i32 {
        match self {
            Dir::Bottom => 1,
            Dir::Top => -1,
            _ => 0,
        }
    }

    fn all() -> [Dir; 4] {
        [Dir::Top, Dir::Right, Dir::Bottom, Dir::Left]
    }

    fn inverse(&self) -> Self {
        match self {
            Dir::Top => Dir::Bottom,
            Dir::Right => Dir::Left,
            Dir::Bottom => Dir::Top,
            Dir::Left => Dir::Right,
        }
    }
}

enum CrucibleType {
    Normal,
    Super,
}

fn dijkstra(
    map: &[Vec<u32>],
    start: (i32, i32),
    end: (i32, i32),
    crucible_type: CrucibleType,
) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    heap.push(Crucible {
        heat_loss: 0,
        pos: start,
        dir: Dir::Right,
        consecultive_dir: 1,
    });

    while let Some(node) = heap.pop() {
        if node.pos == end
            && (matches!(crucible_type, CrucibleType::Normal) || node.consecultive_dir >= 4)
        {
            return node.heat_loss;
        }

        let successors = if matches!(crucible_type, CrucibleType::Normal) {
            node.get_successors(map)
        } else {
            node.get_super_successors(map)
        };

        for successor in successors.into_iter().flatten() {
            if seen.contains(&(successor.consecultive_dir, successor.dir, successor.pos)) {
                continue;
            } else {
                seen.insert((successor.consecultive_dir, successor.dir, successor.pos));

                heap.push(successor);
            }
        }
    }

    0
}

fn part01(input: &str) -> u64 {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let end_x = map[0].len() as i32 - 1;
    let end_y = map.len() as i32 - 1;

    dijkstra(&map, (0, 0), (end_x, end_y), CrucibleType::Normal) as u64
}

fn part02(input: &str) -> u64 {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let end_x = map[0].len() as i32 - 1;
    let end_y = map.len() as i32 - 1;

    dijkstra(&map, (0, 0), (end_x, end_y), CrucibleType::Super) as u64
}

fn main() {
    let input = include_str!("../input/day17.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {
    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part01() {
        assert_eq!(super::part01(INPUT), 102);
    }

    #[test]
    fn part02() {
        assert_eq!(super::part02(INPUT), 94);
    }

    #[test]
    fn part02_2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(super::part02(input), 71);
    }
}

