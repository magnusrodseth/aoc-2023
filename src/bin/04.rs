use std::collections::{HashMap, VecDeque};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Card {
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Card {
    fn get_matches(&self) -> u32 {
        self.your_numbers
            .iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .count() as u32
    }

    fn get_points(&self) -> u32 {
        let winning_numbers = self.get_matches();

        match winning_numbers {
            0 => 0,
            1 => 1,
            _ => {
                let mut points = 1;
                for _ in 1..winning_numbers {
                    points *= 2;
                }
                points
            }
        }
    }

    fn parse_cards(input: &str) -> Vec<Card> {
        input
            .lines()
            .map(|line| line.split(": ").nth(1).expect("Invalid input"))
            .map(|line| line.split(" | ").collect::<Vec<&str>>())
            .map(|parts| {
                let winning_numbers = parts[0]
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|n| n.trim().parse().expect("Invalid input"))
                    .collect::<Vec<u32>>();

                let your_numbers = parts[1]
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|n| n.trim().parse().expect("Invalid input"))
                    .collect::<Vec<u32>>();

                return Card {
                    winning_numbers,
                    your_numbers,
                };
            })
            .collect::<Vec<Card>>()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Card::parse_cards(input)
            .iter()
            .map(|card| card.get_points())
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = Card::parse_cards(input);
    let mut counts: HashMap<usize, u32> = HashMap::new();
    let mut match_memo: HashMap<usize, usize> = HashMap::new();
    let mut backlog: VecDeque<usize> = (0..cards.len()).collect();

    while let Some(card_index) = backlog.pop_front() {
        let matches = *match_memo
            .entry(card_index)
            .or_insert_with(|| cards[card_index].get_matches() as usize);

        *counts.entry(card_index).or_insert(0) += 1;

        for i in 1..=matches {
            let next_card_index = card_index + i;
            if next_card_index < cards.len() {
                backlog.push_back(next_card_index);
            }
        }
    }

    Some(counts.values().sum())
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(30));
    }
}
