use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
enum CellValue {
    Symbol(char),
    Number(u32),
    Period,
}

#[derive(Debug)]
struct Cell {
    value: CellValue,
    row: usize,
    col: usize,
}

impl Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellValue::Symbol(c) => write!(f, "{}", c),
            CellValue::Number(n) => write!(f, "{}", n),
            CellValue::Period => write!(f, "."),
        }
    }
}

impl Cell {
    fn is_symbol(&self) -> bool {
        match self.value {
            CellValue::Symbol(_) => true,
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        match self.value {
            CellValue::Number(_) => true,
            _ => false,
        }
    }

    fn is_period(&self) -> bool {
        match self.value {
            CellValue::Period => true,
            _ => false,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut part_numbers: HashSet<u32> = HashSet::new();

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, c)| Cell {
                    value: match c {
                        '.' => CellValue::Period,
                        '0'..='9' => CellValue::Number(c.to_digit(10).expect("Invalid digit")),
                        _ => CellValue::Symbol(*c),
                    },
                    row,
                    col,
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            let cell = &grid[row][col];

            if cell.is_symbol() {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }

                        let new_row = row as i32 + i;
                        let new_col = col as i32 + j;

                        if new_row < 0
                            || new_row >= rows as i32
                            || new_col < 0
                            || new_col >= cols as i32
                        {
                            continue;
                        }

                        let neighbor = &grid[new_row as usize][new_col as usize];
                        if neighbor.is_number() {
                            let parsed = parse_number(&grid, neighbor);
                            part_numbers.insert(parsed);
                        }
                    }
                }
            }
        }
    }

    // Return the sum of all parts
    Some(part_numbers.iter().sum())
}

/// Navigate horizontally left and right to figure out the start and end of a number cell.
/// Return the parsed number.
fn parse_number(grid: &Vec<Vec<Cell>>, cell: &Cell) -> u32 {
    let mut start = cell.col;
    let mut end = cell.col;

    // Move left
    while start > 0 {
        start -= 1;

        if grid[cell.row][start].is_period() || grid[cell.row][start].is_symbol() {
            break;
        }
    }

    // Move right
    while end < grid[0].len() - 1 {
        end += 1;

        if grid[cell.row][end].is_period() || grid[cell.row][end].is_symbol() {
            break;
        }
    }

    let mut number = String::new();

    for col in start..=end {
        if grid[cell.row][col].is_number() {
            number.push(
                grid[cell.row][col]
                    .value
                    .to_string()
                    .chars()
                    .next()
                    .unwrap(),
            );
        }
    }

    number.parse::<u32>().expect("Invalid number")
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
