use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Eq)]
struct Node {
    heat_loss: u32,
    pos: (i32, i32),
    lru: [Option<Dir>; 3],
}

impl Node {
    fn new(pos: (i32, i32)) -> Self {
        Self {
            pos,
            heat_loss: 0,
            lru: Default::default(),
        }
    }
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss == other.heat_loss && self.pos == other.pos
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl std::cmp::PartialOrd for Node {
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
            Dir::Right => 1 + 1,
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
}

fn get_neighbors(map: &[Vec<u32>], pos: (i32, i32), lru: [Option<Dir>; 3]) -> [Option<Dir>; 3] {
    let width = map.first().unwrap().len();
    let height = map.len();

    let mut neighbors = [None; 3];
    let mut idx = 0;

    for dir in Dir::all() {
        let neighbor = (dir.x() + pos.0, dir.y() + pos.1);

        if neighbor.0 < 0
            || neighbor.1 < 0
            || neighbor.0 as usize >= width
            || neighbor.1 as usize >= height
        {
            continue;
        }

        if let Some(parent_dir) = lru[0] {
            if parent_dir == dir {
                continue;
            }
        }

        if lru.iter().flatten().all(|&last| last == dir) {
            continue;
        }

        neighbors[idx] = Some(dir);
        idx += 1;
    }

    neighbors
}

fn dijkstra(map: &[Vec<u32>], start: (i32, i32), end: (i32, i32)) -> u32 {
    let mut cached_heat_losses = HashMap::new();
    let mut heap = BinaryHeap::new();

    let width = map.first().unwrap().len();
    let height = map.len();

    cached_heat_losses.insert((Default::default(), start), 0);
    heap.push(Node::new(start));

    while let Some(Node {
        pos,
        heat_loss,
        lru,
    }) = heap.pop()
    {
        if pos == end {
            return heat_loss;
        }

        for neighbor_dir in get_neighbors(map, pos, lru).into_iter().flatten() {
            let neighbor_pos = (pos.0 + neighbor_dir.x(), pos.1 + neighbor_dir.y());
            let neighbor_heat_loss = map[neighbor_pos.1 as usize][neighbor_pos.0 as usize];
            let neighbor_lru = [Some(neighbor_dir), lru[0], lru[1]];

            let new_heat_loss = neighbor_heat_loss + heat_loss;

            if let Some(existing_heat_loss) =
                cached_heat_losses.get_mut(&(neighbor_lru, neighbor_pos))
            {
                if new_heat_loss < *existing_heat_loss {
                    *existing_heat_loss = new_heat_loss;
                }
            } else {
                cached_heat_losses.insert((neighbor_lru, neighbor_pos), new_heat_loss);
            }

            heap.push(Node {
                lru: neighbor_lru,
                pos: neighbor_pos,
                heat_loss: new_heat_loss,
            });
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

    let end_x = map.first().unwrap().len() as i32 - 1;
    let end_y = map.len() as i32 - 1;

    dijkstra(&map, (0, 0), (end_x, end_y)) as u64
}

fn part02(_input: &str) -> u64 {
    0
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
        assert_eq!(super::part02(INPUT), 0);
    }
}

