fn part01(input: &str) -> u32 {
    const RED_CUBES: u32 = 12;
    const GREEN_CUBES: u32 = 13;
    const BLUE_CUBES: u32 = 14;

    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();
            let game_number = left.trim_start_matches("Game ").parse::<u32>().unwrap();

            let impossible = right.trim().split(';').any(|subset| {
                subset.trim().split(',').any(|pair| {
                    let (cube_count, cube_color) = pair.trim().split_once(' ').unwrap();
                    let cube_count = cube_count.parse::<u32>().unwrap();
                    match cube_color {
                        "red" => cube_count > RED_CUBES,
                        "green" => cube_count > GREEN_CUBES,
                        "blue" => cube_count > BLUE_CUBES,
                        _ => unreachable!(),
                    }
                })
            });

            if impossible {
                0
            } else {
                game_number
            }
        })
        .sum()
}

fn part02(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, right) = line.split_once(':').unwrap();

            let (min_r, min_g, min_b) = right
                .trim()
                .split(';')
                .map(|subset| {
                    subset
                        .trim()
                        .split(',')
                        .map(|pair| {
                            let (cube_count, cube_color) = pair.trim().split_once(' ').unwrap();
                            let cube_count = cube_count.parse::<u32>().unwrap();
                            match cube_color {
                                "red" => (cube_count, 0, 0),
                                "green" => (0, cube_count, 0),
                                "blue" => (0, 0, cube_count),
                                _ => unreachable!(),
                            }
                        })
                        .fold((0, 0, 0), |(max_r, max_g, max_b), (r, g, b)| {
                            (max_r.max(r), max_g.max(g), max_b.max(b))
                        })
                })
                .fold((0, 0, 0), |(max_r, max_g, max_b), (r, g, b)| {
                    (max_r.max(r), max_g.max(g), max_b.max(b))
                });

            min_r * min_g * min_b
        })
        .sum()
}

fn main() {
    let input = include_str!("../input/day02.input");
    println!("Part01: {}", part01(input));
    println!("Part02: {}", part02(input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part01() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(super::part01(input), 8);
    }

    #[test]
    fn part02() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(super::part02(input), 2286);
    }
}
