use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct PartNumber<'a>(pub usize, pub &'a str);

impl<'a> PartNumber<'a> {
    fn number(&self) -> u32 {
        self.1.parse::<u32>().unwrap()
    }

    fn intersects(&self, index: usize) -> bool {
        index >= self.0 && index <= (self.0 + self.1.len() - 1)
    }
}

fn part02(input: &str) -> u32 {
    let column_count = input.lines().next().unwrap().len();
    let row_count = input.lines().count();

    let input_joined = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>();

    let parts = input
        .lines()
        .enumerate()
        .flat_map(|(line_number, line)| {
            let (_, numbers) = line
                .split(|c: char| c.is_ascii_punctuation())
                .filter(|s| !s.is_empty())
                .fold(
                    (line_number * column_count, vec![]),
                    |(next_index, mut result), number| {
                        let index = input_joined[next_index..].find(number).unwrap() + next_index;
                        let part_number = PartNumber(index, number);
                        result.push(part_number);
                        (index + number.len(), result)
                    },
                );

            numbers
        })
        .collect::<Vec<_>>();

    input_joined
        .char_indices()
        .filter_map(|(idx, c)| if c == '*' { Some(idx) } else { None })
        .map(|idx| {
            let (row, column) = ((idx / column_count) as i32, (idx % column_count) as i32);

            let mut adjacent_parts = HashMap::new();

            for row_add in -1..=1 {
                for column_add in -1..=1 {
                    if row_add == 0 && column_add == 0 {
                        continue;
                    }

                    let r = row_add + row;
                    let c = column_add + column;

                    if r < 0 || c < 0 || c as usize >= column_count || r as usize >= row_count {
                        continue;
                    }

                    let part_index = r as usize * (column_count) + c as usize;

                    if let Some(part) = parts.iter().find(|&part| part.intersects(part_index)) {
                        adjacent_parts.insert(part.0, part);
                    }
                }
            }

            if adjacent_parts.len() == 2 {
                adjacent_parts
                    .into_values()
                    .map(|part| part.number())
                    .reduce(|acc, a| acc * a)
                    .unwrap()
            } else {
                0
            }
        })
        .sum()
}

fn part01(input: &str) -> u32 {
    let numbers = input
        .lines()
        .flat_map(|line| {
            line.split(|c: char| c.is_ascii_punctuation())
                .filter(|s| !s.is_empty())
        })
        .collect::<Vec<_>>();

    let column_count = input.lines().next().unwrap().len();
    let row_count = input.lines().count();

    let input = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>();

    let mut next_index = 0;
    numbers
        .into_iter()
        .filter_map(|n| {
            let n_len = n.len() as i32;

            let relative_index = input[next_index..].find(n).unwrap();
            let index = next_index + relative_index;
            next_index = index + n_len as usize;

            let (row, column) = ((index / column_count) as i32, (index % column_count) as i32);

            for row_add in -1..=1 {
                for column_add in -1..=n_len {
                    let r = row_add + row;
                    let c = column_add + column;

                    if r < 0 || c < 0 || c as usize >= column_count || r as usize >= row_count {
                        continue;
                    }

                    let symbol_index = r as usize * (column_count) + c as usize;
                    let adjacent = input.chars().nth(symbol_index).unwrap();

                    if adjacent.is_ascii_punctuation() && adjacent != '.' {
                        return n.parse::<u32>().ok();
                    }
                }
            }

            None
        })
        .sum()
}

fn main() {
    let input = include_str!("../input/day03.input");
    println!("Part01: {}", part01(input));
    println!("Part02: {}", part02(input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part01() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(super::part01(input), 4361);
    }

    #[test]
    fn part02() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(super::part02(input), 467835);
    }
}
