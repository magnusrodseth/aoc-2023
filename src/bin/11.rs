struct Position {
    row: usize,
    col: usize,
}

type Grid = Vec<Vec<u8>>;

fn parse_input(input: &str) -> (Grid, Vec<Position>) {
    let mut galaxies = vec![];

    let space = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                // Determine the position of the galaxies
                .map(|(col, b)| {
                    if b == b'#' {
                        galaxies.push(Position { row, col });
                        1
                    } else {
                        0
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Grid>();

    (space, galaxies)
}

fn get_empty_rows_cols(space: &[Vec<u8>]) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];
    let (rows, cols) = (space.len(), space[0].len());

    for row in 0..rows {
        if space[row].iter().all(|&c| c == 0) {
            empty_rows.push(row);
        }
    }

    for col in 0..cols {
        if (0..rows).all(|r| space[r][col] == 0) {
            empty_cols.push(col);
        }
    }

    (empty_rows, empty_cols)
}

/// Calculate the distance between all galaxies
/// The distance between two galaxies is the sum of the distance between their rows and columns
/// plus the number of empty rows and columns between them times the expansion factor
fn calculate_distance(
    galaxies: &[Position],
    empty_rows: &[usize],
    empty_cols: &[usize],
    expansion: usize,
) -> usize {
    galaxies
        .iter()
        .enumerate()
        .map(
            |(
                id,
                &Position {
                    row: row_a,
                    col: col_a,
                },
            )| {
                galaxies
                    .iter()
                    .skip(id + 1)
                    .map(
                        |&Position {
                             row: row_b,
                             col: col_b,
                         }| {
                            let mut distance = row_a.abs_diff(row_b) + col_a.abs_diff(col_b);

                            distance += (row_a + 1..row_b)
                                .filter(|&row_a| empty_rows.contains(&row_a))
                                .count()
                                * (expansion - 1);

                            distance += (col_a.min(col_b) + 1..col_a.max(col_b))
                                .filter(|&col_a| empty_cols.contains(&col_a))
                                .count()
                                * (expansion - 1);

                            distance
                        },
                    )
                    .sum::<usize>()
            },
        )
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (space, galaxies) = parse_input(input);
    let (empty_rows, empty_cols) = get_empty_rows_cols(&space);

    Some(calculate_distance(&galaxies, &empty_rows, &empty_cols, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (space, galaxies) = parse_input(input);
    let (empty_rows, empty_cols) = get_empty_rows_cols(&space);

    Some(calculate_distance(
        &galaxies,
        &empty_rows,
        &empty_cols,
        1_000_000,
    ))
}

advent_of_code::main!(11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, None);
    }
}
