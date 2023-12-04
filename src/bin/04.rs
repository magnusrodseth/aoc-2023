#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Card {
    /// Winning numbers is the amount of numbers that are the same in both
    /// your numbers and the winning numbers.
    fn winning_numbers(&self) -> u32 {
        let mut winning_numbers = 0;

        for your_number in &self.your_numbers {
            for winning_number in &self.winning_numbers {
                if your_number == winning_number {
                    winning_numbers += 1;
                }
            }
        }

        winning_numbers
    }

    fn get_points(&self) -> u32 {
        let winning_numbers = self.winning_numbers();

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
    None
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
        assert_eq!(result, None);
    }
}
