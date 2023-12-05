fn part01(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, content) = line.split_once(':').unwrap();

            let (winning_cards, my_cards) = content.trim().split_once('|').unwrap();
            let winning_cards = winning_cards
                .trim()
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let my_cards = my_cards
                .trim()
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let count = my_cards
                .iter()
                .filter(|n| winning_cards.contains(n))
                .count() as u32;

            if count == 0 {
                count
            } else {
                2u32.pow(count - 1)
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../input/day04.input");
    println!("Part 01: {}", part01(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(super::part01(input), 13);
    }
}
