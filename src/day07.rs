use std::{collections::HashMap, fmt::Display};

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum CamelCard {
    #[default]
    Joker = 1,
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Display for CamelCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CamelCard::Two => write!(f, "{}", '2'),
            CamelCard::Three => write!(f, "{}", '3'),
            CamelCard::Four => write!(f, "{}", '4'),
            CamelCard::Five => write!(f, "{}", '5'),
            CamelCard::Six => write!(f, "{}", '6'),
            CamelCard::Seven => write!(f, "{}", '7'),
            CamelCard::Eight => write!(f, "{}", '8'),
            CamelCard::Nine => write!(f, "{}", '9'),
            CamelCard::T => write!(f, "{}", 'T'),
            CamelCard::J => write!(f, "{}", 'J'),
            CamelCard::Q => write!(f, "{}", 'Q'),
            CamelCard::K => write!(f, "{}", 'K'),
            CamelCard::A => write!(f, "{}", 'A'),
            CamelCard::Joker => write!(f, "{}", 'J'),
        }
    }
}

impl CamelCard {
    fn new_with_joker(char: char) -> Self {
        match char {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::Joker,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => unreachable!(),
        }
    }

    fn new(char: char) -> Self {
        match char {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord)]
enum Hand {
    HighCard([CamelCard; 5]),
    OnePair([CamelCard; 5]),
    TwoPair([CamelCard; 5]),
    ThreeOfAKind([CamelCard; 5]),
    FullHouse([CamelCard; 5]),
    FourOfAKind([CamelCard; 5]),
    FiveOfAKind([CamelCard; 5]),
}

#[derive(Default)]
enum Joker {
    #[default]
    Ignore,
    Use,
}

impl Hand {
    fn parse(value: &str, joker: Joker) -> Self {
        let cards = value.chars().take(5).enumerate().fold(
            [CamelCard::default(); 5],
            |mut acc, (idx, c)| {
                let card = match joker {
                    Joker::Ignore => CamelCard::new(c),
                    Joker::Use => CamelCard::new_with_joker(c),
                };
                acc[idx] = card;
                acc
            },
        );

        Hand::new(cards)
    }

    fn new(cards: [CamelCard; 5]) -> Self {
        let card_map = cards.iter().fold(HashMap::new(), |mut map, &card| {
            *map.entry(card).or_insert(0u32) += 1;
            map
        });

        let (unique_cards, max_count, min_count) = (
            card_map.len(),
            card_map.values().max().unwrap(),
            card_map.values().min().unwrap(),
        );

        let hand = match (unique_cards, max_count, min_count) {
            (1, 5, _) => Self::FiveOfAKind(cards),
            (2, 4, 1) => Self::FourOfAKind(cards),
            (2, 3, 2) => Self::FullHouse(cards),
            (3, 3, 1) => Self::ThreeOfAKind(cards),
            (3, 2, 1) => Self::TwoPair(cards),
            (4, 2, 1) => Self::OnePair(cards),
            _ => Self::HighCard(cards),
        };

        if hand.joker_count() > 0 {
            hand.upgrade()
        } else {
            hand
        }
    }

    fn cards(&self) -> &[CamelCard; 5] {
        match self {
            Hand::HighCard(cards) => cards,
            Hand::OnePair(cards) => cards,
            Hand::TwoPair(cards) => cards,
            Hand::ThreeOfAKind(cards) => cards,
            Hand::FourOfAKind(cards) => cards,
            Hand::FiveOfAKind(cards) => cards,
            Hand::FullHouse(cards) => cards,
        }
    }

    fn rank(&self) -> u32 {
        match self {
            Hand::HighCard(_) => 1,
            Hand::OnePair(_) => 2,
            Hand::TwoPair(_) => 3,
            Hand::ThreeOfAKind(_) => 4,
            Hand::FullHouse(_) => 5,
            Hand::FourOfAKind(_) => 6,
            Hand::FiveOfAKind(_) => 7,
        }
    }

    fn joker_count(&self) -> usize {
        self.cards()
            .into_iter()
            .filter(|c| matches!(c, CamelCard::Joker))
            .count()
    }

    fn upgrade(self) -> Hand {
        if self.joker_count() == 5 {
            return self;
        }

        let card_map = self
            .cards()
            .into_iter()
            .filter(|c| !matches!(c, CamelCard::Joker))
            .fold(HashMap::new(), |mut map, &card| {
                *map.entry(card).or_insert(0u32) += 1;
                map
            });

        let (best_card, _) = card_map
            .into_iter()
            .max_by(|(c_a, cnt_a), (c_b, cnt_b)| {
                if cnt_a == cnt_b {
                    c_a.cmp(c_b)
                } else {
                    cnt_a.cmp(cnt_b)
                }
            })
            .unwrap();

        let new_cards = self.cards().into_iter().enumerate().fold(
            [Default::default(); 5],
            |mut cards, (idx, &card)| {
                cards[idx] = if matches!(card, CamelCard::Joker) {
                    best_card
                } else {
                    card
                };

                cards
            },
        );

        let updated = Hand::new(new_cards);

        match updated {
            Hand::OnePair(_) => Self::OnePair(self.take_cards()),
            Hand::TwoPair(_) => Self::TwoPair(self.take_cards()),
            Hand::ThreeOfAKind(_) => Self::ThreeOfAKind(self.take_cards()),
            Hand::FullHouse(_) => Self::FullHouse(self.take_cards()),
            Hand::FourOfAKind(_) => Self::FourOfAKind(self.take_cards()),
            Hand::FiveOfAKind(_) => Self::FiveOfAKind(self.take_cards()),
            _ => unreachable!(),
        }
    }

    fn take_cards(self) -> [CamelCard; 5] {
        match self {
            Hand::HighCard(cards) => cards,
            Hand::OnePair(cards) => cards,
            Hand::TwoPair(cards) => cards,
            Hand::ThreeOfAKind(cards) => cards,
            Hand::FullHouse(cards) => cards,
            Hand::FourOfAKind(cards) => cards,
            Hand::FiveOfAKind(cards) => cards,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hand::HighCard(cards) => {
                write!(f, "HighCard ")?;
                cards.iter().try_for_each(|card| write!(f, "{}", card))
            }
            Hand::OnePair(cards) => {
                write!(f, "OnePair ")?;
                cards.iter().try_for_each(|card| write!(f, "{}", card))
            }
            Hand::TwoPair(cards) => {
                write!(f, "TwoPair ")?;
                cards.iter().try_for_each(|card| write!(f, "{}", card))
            }
            Hand::ThreeOfAKind(cards) => {
                write!(f, "ThreeOfAKind ")?;
                cards.iter().try_for_each(|card| write!(f, "{}", card))
            }
            Hand::FourOfAKind(cards) => {
                write!(f, "FourOfAKind ")?;
                cards.iter().try_for_each(|card| write!(f, "{}", card))
            }
            Hand::FiveOfAKind(cards) => {
                write!(f, "FiveOfAKind ")?;
                cards.iter().try_for_each(|card| write!(f, "{}", card))
            }
            Hand::FullHouse(cards) => {
                write!(f, "FullHouse ")?;
                cards.iter().try_for_each(|card| write!(f, "{}", card))
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank().eq(&other.rank())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            let cards = self.cards();
            let other_cards = other.cards();

            for i in 0..cards.len() {
                if cards[i] == other_cards[i] {
                    continue;
                } else {
                    return cards[i].partial_cmp(&other_cards[i]);
                }
            }

            Some(std::cmp::Ordering::Equal)
        } else {
            self.rank().partial_cmp(&other.rank())
        }
    }
}

fn part01(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let hand: Hand = Hand::parse(cards, Joker::Ignore);
            let bid = bid.parse::<u64>().unwrap();

            (hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

    hands
        .into_iter()
        .enumerate()
        .fold(0, |mut sum, (rank, (_, bid))| {
            sum += (rank + 1) as u64 * bid;
            sum
        })
}

fn part02(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let hand: Hand = Hand::parse(cards, Joker::Use);
            let bid = bid.parse::<u64>().unwrap();

            (hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

    hands
        .into_iter()
        .enumerate()
        .fold(0, |mut sum, (rank, (_, bid))| {
            sum += (rank + 1) as u64 * bid;
            sum
        })
}

fn main() {
    let input = include_str!("../input/day07.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

        assert_eq!(super::part01(input), 6440);
    }

    #[test]
    fn part02() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!(super::part02(input), 5905);
    }
}
