const MAX_RED_CUBES: usize = 12;
const MAX_GREEN_CUBES: usize = 13;
const MAX_BLUE_CUBES: usize = 14;

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Cube {
    color: Color,
    count: usize,
}

#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = input
        .lines()
        .map(|line| line.split(": ").collect::<Vec<&str>>())
        .map(|game| {
            let id = game[0]
                .split(" ")
                .last()
                .expect("Must have format 'Game [number]'")
                .parse::<u32>()
                .expect("Game ID must be a number");

            dbg!(&id);

            let rounds = game[1]
                .split("; ")
                .map(|round| {
                    round
                        .split(", ")
                        // At this point, the split is on the format ["6 red", "1 blue", "3 green"]
                        // We want to split each of these on the space,
                        // and then parse the first part as a number and second part as a color.
                        .map(|cube| {
                            let mut cube = cube.split(" ");
                            let count = cube
                                .next()
                                .expect("Cube must have a count")
                                .parse::<usize>()
                                .expect("Cube count must be a number");

                            let color = match cube
                                .next()
                                .expect("Cube must have a color")
                                .to_lowercase()
                                .as_str()
                            {
                                "red" => Color::Red,
                                "green" => Color::Green,
                                "blue" => Color::Blue,
                                _ => panic!("Invalid color"),
                            };

                            Cube { count, color }
                        })
                        .collect::<Vec<Cube>>()
                })
                .map(|cubes| Round { cubes })
                .collect::<Vec<Round>>();

            Game { id, rounds }
        })
        .collect::<Vec<Game>>();

    // Now we want to filter out only valid games.
    // A valid game is one where:
    // For all rounds, the number of red, green and blue cubes is less than the max number of cubes for that color.
    let valid_game_ids = games
        .iter()
        .filter(|game| {
            game.rounds.iter().all(|round| {
                round.cubes.iter().all(|cube| match cube.color {
                    Color::Red => cube.count <= MAX_RED_CUBES,
                    Color::Green => cube.count <= MAX_GREEN_CUBES,
                    Color::Blue => cube.count <= MAX_BLUE_CUBES,
                })
            })
        })
        .map(|game| game.id)
        .collect::<Vec<u32>>();

    dbg!(&valid_game_ids);

    // Sum all the valid game IDs
    Some(valid_game_ids.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, None);
    }
}
