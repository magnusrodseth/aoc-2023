use core::panic;

use pathfinding::matrix::Matrix;

/// A position on the grid, with a row and column.
type Position = (usize, usize);

/// Parses the input string into a starting position and a grid matrix
fn parse(input: &str) -> (Position, Matrix<u8>) {
    let direct = Matrix::from_rows(input.lines().map(|s| s.bytes())).unwrap();

    // Create a new matrix with padding to handle edges
    let mut raw = Matrix::new(direct.rows + 2, direct.columns + 2, b'.');
    raw.set_slice((1, 1), &direct);

    for (row, col) in raw.keys() {
        let is_starting_position = raw[(row, col)] == b'S';

        if is_starting_position {
            /// Check if a symbol matches any of the given bytes of other symbols.
            macro_rules! check {
                {$raw:expr, $pos:expr, $bit:expr, $($checks:expr),*} => {
                    if matches!($raw.get($pos), Some($($checks)|*)) { 1 << $bit } else { 0 }
                }
            }

            // Calculate the correct bend based on adjacent tiles
            let bend = check!(raw, (row - 1, col), 3, b'|', b'F', b'7')
                | check!(raw, (row + 1, col), 2, b'|', b'L', b'J')
                | check!(raw, (row, col + 1), 1, b'-', b'7', b'J')
                | check!(raw, (row, col - 1), 0, b'-', b'L', b'F');

            // Determine the real tile type and assert if it's invalid
            let real = b"XXX-X7FXXJLX|XXX"[bend];
            assert!(real != b'X', "Invalid bend {bend:04b}");

            raw[(row, col)] = real;

            // Return the starting position and the updated grid
            return ((row, col), raw);
        }
    }
    panic!("Invalid input");
}

/// Cycles through the path starting from `start`
fn cycle(start: Position, grid: &Matrix<u8>) -> impl Iterator<Item = Position> + '_ {
    let mut position = start;

    let mut direction = match grid[start] {
        b'|' | b'F' | b'7' => (1, 0),
        b'J' | b'L' => (-1, 0),
        _ => (0, 1),
    };

    std::iter::once(start).chain(std::iter::from_fn(move || {
        position = grid.move_in_direction(position, direction).unwrap();
        (position != start).then(|| {
            // Update direction based on current tile and its type
            direction = match (direction, grid[position]) {
                ((1, 0), b'L') | ((-1, 0), b'F') => (0, 1),
                ((1, 0), b'J') | ((-1, 0), b'7') => (0, -1),
                ((0, 1), b'7') | ((0, -1), b'F') => (1, 0),
                ((0, 1), b'J') | ((0, -1), b'L') => (-1, 0),
                _ => direction,
            };
            position
        })
    }))
}

// Parses barriers from the grid based on the cycle starting at `start`
fn parse_barriers(start: Position, grid: &Matrix<u8>) -> Matrix<u8> {
    let mut barriers = Matrix::new(grid.rows, grid.columns, b'.');
    for pos in cycle(start, &grid) {
        // Mark the cycle path on the barriers matrix
        barriers[pos] = grid[pos];
    }

    barriers
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, grid) = parse(input);
    Some((cycle(start, &grid).count() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start, grid) = parse(input);
    let mut inside = false;

    let enclosed_tiles = parse_barriers(start, &grid)
        .items()
        .filter(|&((_, c), &i)| {
            inside &= c != 0;
            inside ^= matches!(i, b'|' | b'J' | b'L');
            inside && i == b'.'
        })
        .count();

    Some(enclosed_tiles as u32)
}

advent_of_code::main!(10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_simple() {
        let result = part_one(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_complex() {
        let result = part_one(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_simple() {
        let result = part_two(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_complex() {
        let result = part_two(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, Some(8));
    }
}
