fn expand(input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];

    for line in input.lines() {
        grid.push(line);
        if line.chars().all(|c| c == '.') {
            grid.push(line);
        }
    }

    let width = grid.first().unwrap().len();
    let mut new_grid: Vec<Vec<_>> = vec![Default::default(); grid.len()];

    for i in 0..width {
        let repeat = grid.iter().all(|line| line.chars().nth(i).unwrap() == '.');
        for row in 0..grid.len() {
            new_grid[row].push(grid[row].chars().nth(i).unwrap());
            if repeat {
                new_grid[row].push('.');
            }
        }
    }

    new_grid
}

fn part01(input: &str) -> u64 {
    let grid = expand(input);
    let galaxies = grid
        .iter()
        .enumerate()
        .flat_map(|(y, chars)| {
            chars.iter().enumerate().filter_map(
                move |(x, &c)| {
                    if c == '#' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .collect::<Vec<_>>();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(idx, source)| {
            galaxies.iter().skip(idx + 1).map(move |dest| {
                let x_offset = (dest.0 as i32 - source.0 as i32).abs();
                let y_offset = (dest.1 as i32 - source.1 as i32).abs();
                (x_offset + y_offset) as u64
            })
        })
        .sum()
}

fn part02(_input: &str) -> u64 {
    0
}

fn main() {
    let input = include_str!("../input/day11.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(super::part01(input), 374);
    }

    #[test]
    fn part02() {
        let input = "";

        assert_eq!(super::part02(input), 0);
    }
}

