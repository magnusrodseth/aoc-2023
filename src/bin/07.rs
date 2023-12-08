use std::{collections::HashMap, fmt::Display};

fn card_strength(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '2'..='9' => card.to_digit(10).expect("Invalid card"),
        _ => 0, // Invalid card
    }
}

fn card_strength_with_joker(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '2'..='9' => card.to_digit(10).expect("Invalid card"),
        'J' => 1, // J is now the weakest
        _ => 0,   // Invalid card
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|&card| match card {
                14 => 'A',
                13 => 'K',
                12 => 'Q',
                11 | 1 => 'J',
                10 => 'T',
                2..=9 => card.to_string().chars().next().unwrap(),
                _ => '?',
            })
            .collect::<String>();

        let card_type = match self.hand_type_with_jokers() {
            HandType::HighCard => "High Card",
            HandType::OnePair => "One Pair",
            HandType::TwoPair => "Two Pair",
            HandType::ThreeOfAKind => "Three of a Kind",
            HandType::FullHouse => "Full House",
            HandType::FourOfAKind => "Four of a Kind",
            HandType::FiveOfAKind => "Five of a Kind",
        };

        write!(f, "{} ({})", cards, card_type)
    }
}

impl Hand {
    const JOKER_VALUE: u32 = 1;

    fn hand_type(&self) -> HandType {
        match self
            .card_frequencies()
            .values()
            .collect::<Vec<&u32>>()
            .as_slice()
        {
            [5] => HandType::FiveOfAKind,
            [4, 1] | [1, 4] => HandType::FourOfAKind,
            [3, 2] | [2, 3] => HandType::FullHouse,
            [3, 1, 1] | [1, 3, 1] | [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 2, 1] | [2, 1, 2] | [1, 2, 2] => HandType::TwoPair,
            [2, 1, 1, 1] | [1, 2, 1, 1] | [1, 1, 2, 1] | [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn hand_type_with_jokers(&self) -> HandType {
        // Count jokers
        let joker_count = self
            .cards
            .iter()
            .filter(|&&card| card == Hand::JOKER_VALUE)
            .count() as u32;

        // If there are jokers, we need to determine the best possible hand
        if joker_count > 0 {
            return self.best_hand_with_jokers(joker_count);
        }

        // If no jokers, use the original method
        self.hand_type()
    }

    fn best_hand_with_jokers(&self, joker_count: u32) -> HandType {
        let mut frequencies = self.card_frequencies();

        // Remove the frequency of Jokers for analysis
        frequencies.remove(&Hand::JOKER_VALUE);

        // Start from the strongest hand and check if it can be formed with available jokers
        if joker_count == 5 {
            return HandType::FiveOfAKind; // All jokers can form any hand
        }

        // Check for Five of a Kind
        if let Some(&max_freq) = frequencies.values().max() {
            if max_freq + joker_count >= 5 {
                return HandType::FiveOfAKind;
            }
        }

        if let Some(&max_freq) = frequencies.values().max() {
            if max_freq == 4 || (max_freq == 3 && joker_count >= 2) {
                return HandType::FiveOfAKind;
            }
        }

        // Check for Four of a Kind or Full House
        if let Some(&max_freq) = frequencies.values().max() {
            if max_freq + joker_count >= 4 {
                return HandType::FourOfAKind;
            }
            if max_freq == 3 && joker_count + frequencies.len() as u32 - 1 >= 2 {
                return HandType::FullHouse;
            }
        }

        // Check for Three of a Kind
        if joker_count >= 3 || (frequencies.values().any(|&v| v == 2) && joker_count >= 1) {
            return HandType::ThreeOfAKind;
        }

        // Check for Two Pair
        if frequencies.len() as u32 + joker_count >= 4 {
            return HandType::TwoPair;
        }

        // Check for One Pair
        if joker_count >= 1 || frequencies.values().any(|&v| v == 2) {
            return HandType::OnePair;
        }

        HandType::HighCard
    }

    fn card_frequencies(&self) -> HashMap<u32, u32> {
        let mut frequencies = HashMap::new();

        for value in &self.cards {
            *frequencies.entry(*value).or_insert(0) += 1;
        }

        frequencies
    }

    fn compare_with(&self, other: &Hand) -> std::cmp::Ordering {
        let is_equal_hand_type = self.hand_type() == other.hand_type();

        if !is_equal_hand_type {
            return self.hand_type().cmp(&other.hand_type());
        }

        // If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.
        for (card_a, card_b) in self.cards.iter().zip(other.cards.iter()) {
            if card_a != card_b {
                return card_a.cmp(card_b);
            }
        }

        std::cmp::Ordering::Equal
    }

    fn compare_with_jokers(&self, other: &Hand) -> std::cmp::Ordering {
        let self_type = self.hand_type_with_jokers();
        let other_type = other.hand_type_with_jokers();

        if self_type != other_type {
            return self_type.cmp(&other_type);
        }

        // In case of a tie, compare based on actual card values, considering 'J' as the weakest
        for (card_a, card_b) in self.cards.iter().zip(other.cards.iter()) {
            if card_a != card_b {
                return card_a.cmp(card_b);
            }
        }

        std::cmp::Ordering::Equal
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .map(|(hand, bid)| {
            (
                hand.chars().collect::<Vec<char>>(),
                bid.parse::<u32>().unwrap(),
            )
        })
        .map(|(hand, bid)| Hand {
            cards: hand.iter().map(|card| card_strength(*card)).collect(),
            bid,
        })
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.compare_with(b));

    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank as u32 + 1))
        .sum::<u32>();

    Some(total_winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .map(|(hand, bid)| {
            (
                hand.chars().collect::<Vec<char>>(),
                bid.parse::<u32>().unwrap(),
            )
        })
        .map(|(hand, bid)| Hand {
            cards: hand
                .iter()
                .map(|card| card_strength_with_joker(*card))
                .collect(),
            bid,
        })
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.compare_with_jokers(b));

    for hand in hands.iter() {
        println!("{}", hand);
    }

    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank as u32 + 1))
        .sum::<u32>();

    Some(total_winnings)
    // None
}

advent_of_code::main!(7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_part_two_with_tie() {
        let result = part_two("JKKK2 10\nQQQQ2 20\n");
        assert_eq!(result, Some(10 * 1 + 20 * 2));
    }
}
