use std::collections::{HashMap, HashSet};

const START_LABEL: &str = "AAA";
const END_LABEL: &str = "ZZZ";

type Label = String;
type LeftRight = (Label, Label);

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node {
    label: Label,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn all_labels_end_with_z(labels: &HashSet<String>, nodes: &HashMap<String, Node>) -> bool {
    labels.iter().all(|label| {
        nodes
            .get(label)
            .map_or(false, |node| node.label.ends_with("Z"))
    })
}

fn create_graph(input: &str) -> HashMap<Label, Node> {
    let mut nodes_map: HashMap<String, LeftRight> = HashMap::new();
    input.lines().skip(2).for_each(|line| {
        let parts = line.split(" = ").collect::<Vec<&str>>();
        let label = parts[0].to_string();
        let (left_label, right_label) = parts[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();

        nodes_map.insert(label, (left_label.to_string(), right_label.to_string()));
    });

    let mut nodes: HashMap<Label, Node> = HashMap::new();
    for (label, _) in nodes_map.iter() {
        nodes.insert(
            label.clone(),
            Node {
                label: label.clone(),
                left: None,
                right: None,
            },
        );
    }

    // Second pass to set connections
    for (label, (left_label, right_label)) in nodes_map.iter() {
        let left_node = nodes.get(left_label).map(|n| Box::new(n.clone()));
        let right_node = nodes.get(right_label).map(|n| Box::new(n.clone()));

        if let Some(node) = nodes.get_mut(label) {
            node.left = left_node;
            node.right = right_node;
        }
    }

    nodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let directions =
        Direction::parse_directions(input.lines().nth(0).expect("should have first line"));

    let nodes = create_graph(input);

    let mut current_node = nodes
        .get(START_LABEL)
        .expect("should have start node")
        .clone();

    let mut steps: u32 = 0;
    let mut index = 0;

    while current_node.label != END_LABEL {
        let direction = &directions[index % directions.len()];

        let next_label = match direction {
            Direction::Left => current_node.left.as_ref().map(|n| n.label.clone()),
            Direction::Right => current_node.right.as_ref().map(|n| n.label.clone()),
        };

        current_node = nodes
            .get(&next_label.expect("should have next label"))
            .expect("should have next node")
            .clone();

        steps += 1;
        index += 1;
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let directions =
        Direction::parse_directions(input.lines().nth(0).expect("should have first line"));

    let nodes = create_graph(input);

    let starting_labels = nodes
        .iter()
        .filter(|(_, node)| node.label.ends_with('A'))
        .map(|(_, node)| node.label.clone())
        .collect::<HashSet<String>>();

    let mut visited_labels = starting_labels;
    let mut steps: usize = 0;

    while !all_labels_end_with_z(&visited_labels, &nodes) {
        let mut next_visited_labels = HashSet::new();
        let direction = &directions[steps % directions.len()];

        for label in visited_labels.iter() {
            let node = nodes.get(label).expect("Node must exist for label");
            let next_label = match direction {
                Direction::Left => node.left.as_ref().map(|n| n.label.clone()),
                Direction::Right => node.right.as_ref().map(|n| n.label.clone()),
            };

            if let Some(label) = next_label {
                next_visited_labels.insert(label);
            }
        }

        visited_labels = next_visited_labels;
        steps += 1;
    }

    Some(steps as u32)
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
