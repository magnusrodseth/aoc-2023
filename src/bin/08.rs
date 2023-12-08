use num::integer::lcm as lowest_common_multiple;
use std::collections::HashMap;

const START_LABEL: &str = "AAA";
const END_LABEL: &str = "ZZZ";

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse_directions(input: &str) -> Vec<Self> {
        input
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("invalid direction"),
            })
            .collect::<Vec<Direction>>()
    }
}

type Label = String;
type LeftRight = (Label, Label);

fn create_graph(input: &str) -> HashMap<Label, LeftRight> {
    input
        .lines()
        .skip(2)
        .map(|line| {
            let parts = line.split(" = ").collect::<Vec<&str>>();
            let label = parts[0].to_string();
            let (left_label, right_label) = parts[1]
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();
            (label, (left_label.to_string(), right_label.to_string()))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let directions =
        Direction::parse_directions(input.lines().nth(0).expect("should have first line"));

    let graph = create_graph(input);

    let mut current_label = START_LABEL.to_string();
    let mut steps: u32 = 0;

    while current_label != END_LABEL {
        let direction = &directions[steps as usize % directions.len()];

        current_label = match direction {
            Direction::Left => graph.get(&current_label).unwrap().0.clone(),
            Direction::Right => graph.get(&current_label).unwrap().1.clone(),
        };

        steps += 1;
    }

    Some(steps)
}

fn part_two(input: &str) -> Option<u32> {
    let directions =
        Direction::parse_directions(input.lines().nth(0).expect("should have first line"));

    let graph = create_graph(input);

    // Collect all starting nodes (nodes ending with 'A')
    let start_nodes = graph
        .keys()
        .filter(|&k| k.ends_with('A'))
        .collect::<Vec<&String>>();

    // Calculate the LCM of steps for each starting node
    let result_lcm = start_nodes
        .iter()
        .map(|&start_node| {
            let mut current_label = start_node;
            let mut steps: usize = 0;

            while !current_label.ends_with('Z') {
                let direction = &directions[steps % directions.len()];
                current_label = match direction {
                    Direction::Left => &graph.get(current_label).unwrap().0,
                    Direction::Right => &graph.get(current_label).unwrap().1,
                };
                steps += 1;
            }

            steps as u64
        })
        .fold(1, lowest_common_multiple);

    Some(result_lcm as u32)
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_longer() {
        let result = part_one(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, Some(6));
    }
}
