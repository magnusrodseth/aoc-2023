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

impl Game {
    fn parse(input: Vec<&str>) -> Self {
        let id = input[0]
            .split(" ")
            .last()
            .expect("Must have format `Game [number]`")
            .parse::<u32>()
            .expect("Game ID must be a number");

        let rounds = input[1]
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
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| line.split(": ").collect::<Vec<&str>>())
        .map(|game| Game::parse(game))
        .collect::<Vec<Game>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_games(input);

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

    Some(valid_game_ids.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_games(input);

    let products = games
        .iter()
        // Find max values for each color in each game
        .map(|game| {
            game.rounds
                .iter()
                .fold((0, 0, 0), |(max_red, max_green, max_blue), round| {
                    round.cubes.iter().fold(
                        (max_red, max_green, max_blue),
                        |(max_red, max_green, max_blue), cube| match cube.color {
                            Color::Red => (max_red.max(cube.count), max_green, max_blue),
                            Color::Green => (max_red, max_green.max(cube.count), max_blue),
                            Color::Blue => (max_red, max_green, max_blue.max(cube.count)),
                        },
                    )
                })
        })
        // Find the product of the max values for each game
        .map(|(max_red, max_green, max_blue)| max_red * max_green * max_blue)
        .collect::<Vec<usize>>();

    // Return the sum of all the products
    Some(products.iter().sum::<usize>() as u32)
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
        assert_eq!(result, Some(2286));
    }
}
