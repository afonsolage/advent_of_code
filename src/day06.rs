fn is_hold_time_enough(hold_time: u64, time_limit: u64, max_distance: u64) -> bool {
    (time_limit - hold_time) * hold_time > max_distance
}
fn part01(input: &str) -> u64 {
    let (time, distance) = input.split_once('\n').unwrap();
    let times = time
        .trim_start_matches("Time:")
        .split_ascii_whitespace()
        .filter_map(|entry| entry.parse::<u64>().ok());

    let distance = distance
        .trim_start_matches("Distance:")
        .split_ascii_whitespace()
        .filter_map(|entry| entry.parse::<u64>().ok());

    times
        .zip(distance)
        .map(|(time, distance)| {
            let min = (1..time)
                .into_iter()
                .find(|&hold_time| is_hold_time_enough(hold_time, time, distance))
                .unwrap();
            let max = (1..time)
                .rev()
                .into_iter()
                .find(|&hold_time| is_hold_time_enough(hold_time, time, distance))
                .unwrap();

            let max = max + 1;

            max - min
        })
        .fold(1, |acc, count| acc * count)
}

fn main() {
    let input = include_str!("../input/day06.input");
    println!("Part 01: {}", part01(input));
}

#[cfg(test)]
mod test {
    #[test]
    fn part01() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(super::part01(input), 288);
    }
}
