use std::collections::HashMap;

struct Part {
    symbol: char,
    position: String,
    value: u32,
}

fn parse_part_numbers(lines: &[&str]) -> Vec<Part> {
    let mut parts = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        let mut part_number = String::new();
        let mut part: Option<Part> = None;

        for (col, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                part_number.push(char);
                if let Some(parsed) = parse_part(lines, col, row) {
                    part = Some(parsed);
                }
            }

            let next_char = line.chars().nth(col + 1);

            if next_char.map_or(true, |c| !c.is_digit(10)) {
                if let Some(p) = part.take() {
                    parts.push(Part {
                        value: part_number.parse().unwrap_or(0),
                        ..p
                    });
                }
                part_number.clear();
            }
        }
    }

    parts
}

fn parse_part(lines: &[&str], col: usize, row: usize) -> Option<Part> {
    let directions: Vec<(isize, isize)> = (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .filter(|&(dx, dy)| dx != 0 || dy != 0)
        .collect();

    for (dx, dy) in directions {
        let next_line = lines.get((row as isize + dy) as usize);
        if let Some(line) = next_line {
            let next_char = line.chars().nth((col as isize + dx) as usize);
            if let Some(char) = next_char {
                if char != '.' && !char.is_digit(10) {
                    return Some(Part {
                        symbol: char,
                        position: format!("{}:{}", col as isize + dx, row as isize + dy),
                        value: 0,
                    });
                }
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let parts = parse_part_numbers(&lines);
    Some(parts.iter().map(|p| p.value).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let parts = parse_part_numbers(&lines);
    let filtered_parts: Vec<&Part> = parts.iter().filter(|part| part.symbol == '*').collect();

    let mut part_groups: HashMap<String, Vec<&Part>> = HashMap::new();

    for part in filtered_parts {
        part_groups
            .entry(part.position.clone())
            .or_insert_with(Vec::new)
            .push(part);
    }

    let sum = part_groups
        .values()
        // Find gears, i.e. '*' symbols adjacent to exactly two numbers
        .filter(|parts| parts.len() == 2)
        // Find gear ratio by multiplying the values of each part
        .map(|parts| parts[0].value * parts[1].value)
        // Sum the gear ratios
        .sum();

    Some(sum)
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(467835));
    }
}
