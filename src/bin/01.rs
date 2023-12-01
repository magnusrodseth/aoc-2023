use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    let lines: Vec<&str> = input.split('\n').map(|line| line.trim()).collect();

    for line in lines {
        let mut start_index = 0;
        let mut end_index = line.len() - 1;
        let mut final_start_digit: Option<u32> = None;
        let mut final_end_digit: Option<u32> = None;

        for _ in 0..line.len() {
            let start_character = line.chars().nth(start_index).unwrap();
            let end_character = line.chars().nth(end_index).unwrap();

            // Parse start character to u32
            match start_character.to_digit(10) {
                Some(start_digit) => {
                    final_start_digit = Some(start_digit);
                }
                None => {
                    // Start character is not a digit
                    start_index += 1;
                }
            }

            match end_character.to_digit(10) {
                Some(end_digit) => {
                    final_end_digit = Some(end_digit);
                }
                None => {
                    // End character is not a digit
                    end_index -= 1;
                }
            }
        }

        // Merge the final digits, and parse them to u32
        let number = match (final_start_digit, final_end_digit) {
            (Some(start_digit), Some(end_digit)) => format!("{}{}", start_digit, end_digit)
                .parse::<u32>()
                .expect("Could not parse start and end digit"),
            _ => {
                panic!("Could not parse start and end digit")
            }
        };

        // Add the number to the result
        result += number;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(
        input
            .split("\n")
            .map(|line| replace_digit_words_in_string(line))
            .collect::<Vec<String>>()
            .join("\n")
            .as_str(),
    )
}

fn replace_digit_words_in_string(s: &str) -> String {
    let letter_words: HashMap<&str, char> = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .iter()
    .cloned()
    .collect();

    let mut occurrences = Vec::new();

    // Find all occurrences of digit words and digits
    for (word, digit) in &letter_words {
        occurrences.extend(s.match_indices(word).map(|(index, _)| (index, *digit)));
    }
    for digit in '0'..='9' {
        occurrences.extend(s.match_indices(digit).map(|(index, _)| (index, digit)));
    }

    // Sort by index
    occurrences.sort_by_key(|&(index, _)| index);

    // Join digits into a string
    occurrences.iter().map(|&(_, digit)| digit).collect()
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, Some(281));
    }
}
