use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        // Map over each line, and filter our only the digits
        .map(|line| line.chars().filter(|c| c.is_digit(10)))
        // Map and keep only first and last digit. If there's only one digit, set last to first
        .map(|mut digits| {
            let first = digits.next();
            let last = digits.last().or(first);
            (first, last)
        })
        // Merge first and last into a string, and parse to u32
        .map(|(first, last)| {
            first
                .and_then(|first| last.map(|last| format!("{}{}", first, last)))
                .and_then(|digits| digits.parse::<u32>().ok())
        })
        // Sum all the numbers
        .sum::<Option<u32>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(
        input
            .lines()
            .map(replace_digit_words_in_string)
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
