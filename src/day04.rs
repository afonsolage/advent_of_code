use std::collections::HashMap;

fn count_matching_cards(content: &str) -> u32 {
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

    my_cards
        .iter()
        .filter(|n| winning_cards.contains(n))
        .count() as u32
}

fn part02(input: &str) -> u32 {
    let mut cards = input
        .lines()
        .map(|line| {
            let (card, content) = line.split_once(':').unwrap();
            let card = card
                .matches(|c: char| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            let count = count_matching_cards(content);

            (card, count)
        })
        .collect::<Vec<_>>();

    cards.sort_by(|(a, _), (b, _)| a.cmp(b));

    let card_copies = cards
        .iter()
        .map(|(c, _)| (*c, 1u32))
        .collect::<HashMap<_, _>>();

    let max = *cards.iter().map(|(card, _)| card).max().unwrap();

    cards
        .into_iter()
        .fold(
            (card_copies, 0),
            |(mut card_copies, card_count), (card, winning)| {
                let copies = card_copies.remove(&card).unwrap();
                let end_card = (card + winning).min(max);

                for next_card in (card + 1)..=end_card {
                    *card_copies.get_mut(&next_card).unwrap() += copies;
                }

                (card_copies, card_count + copies)
            },
        )
        .1
}

fn part01(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, content) = line.split_once(':').unwrap();

            let count = count_matching_cards(content);

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
    println!("Part 02: {}", part02(input));
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

    #[test]
    fn part02() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(super::part02(input), 30);
    }
}
