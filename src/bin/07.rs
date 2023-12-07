use std::collections::HashMap;

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

impl Hand {
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
    None
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
        assert_eq!(result, None);
    }
}
