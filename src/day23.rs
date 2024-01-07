use std::collections::{HashMap, HashSet};

const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

trait Slope {
    fn is_slope(&self) -> bool;
    fn is_downhill(&self, dir: (isize, isize)) -> bool;
}

impl Slope for char {
    fn is_slope(&self) -> bool {
        matches!(self, '^' | '>' | 'v' | '<')
    }

    fn is_downhill(&self, dir: (isize, isize)) -> bool {
        match self {
            '^' => dir == DIRS[0],
            '>' => dir == DIRS[1],
            'v' => dir == DIRS[2],
            '<' => dir == DIRS[3],
            _ => false,
        }
    }
}

fn longest_path(
    map: &[Vec<char>],
    begin: (isize, isize),
    end: (isize, isize),
    cache: &mut HashMap<(isize, isize), Vec<(isize, isize)>>,
    walked: &mut HashSet<(isize, isize)>,
) -> Vec<(isize, isize)> {
    let mut steps = vec![];

    let width = map[0].len() as isize;
    let height = map.len() as isize;

    let mut next_steps = vec![begin];

    while next_steps.len() == 1 {
        let step = next_steps.pop().unwrap();

        steps.push(step);
        walked.insert(step);

        if step == end {
            break;
        }

        for dir in DIRS {
            let next_step = (step.0 + dir.0, step.1 + dir.1);
            if next_step.0 < 0
                || next_step.1 < 0
                || next_step.0 >= width
                || next_step.1 >= height
                || walked.contains(&next_step)
            {
                continue;
            }

            let c = map[next_step.1 as usize][next_step.0 as usize];
            if c == '#' || (c.is_slope() && !c.is_downhill(dir)) {
                continue;
            }

            next_steps.push(next_step);
        }
    }

    if !next_steps.is_empty() {
        assert_eq!(next_steps.len(), 2);

        let (begin_a, begin_b) = (next_steps.pop().unwrap(), next_steps.pop().unwrap());

        let path_a = if let Some(cached) = cache.get(&begin_a) {
            cached.clone()
        } else {
            longest_path(map, begin_a, end, cache, &mut walked.clone())
        };

        let path_b = if let Some(cached) = cache.get(&begin_b) {
            cached.clone()
        } else {
            longest_path(map, begin_b, end, cache, walked)
        };

        if path_a.len() > path_b.len() {
            steps.extend_from_slice(&path_a);
        } else {
            steps.extend_from_slice(&path_b);
        }
    }

    let existing = cache.insert(begin, steps.clone());
    assert!(existing.is_none());

    steps
}

fn part01(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = map[0].len() as isize;
    let height = map.len() as isize;

    let begin = (1, 0);
    let end = (width - 2, height - 1);

    let path = longest_path(&map, begin, end, &mut HashMap::new(), &mut HashSet::new());

    path.len() - 1
}

fn part02(_input: &str) -> usize {
    0
}

fn main() {
    let input = include_str!("../input/day23.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {
    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part01() {
        assert_eq!(super::part01(INPUT), 94);
    }

    #[test]
    fn part02() {
        assert_eq!(super::part02(INPUT), 0);
    }
}
