use std::collections::HashMap;

const START_LABEL: &str = "AAA";
const END_LABEL: &str = "ZZZ";

type Label = String;
type LeftRight = (Label, Label);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Node {
    label: Label,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
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

    let mut nodes: HashMap<String, Node> = HashMap::new();
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
    let directions = input
        .lines()
        .nth(0)
        .expect("should have first line")
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction"),
        })
        .collect::<Vec<Direction>>();

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
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, None);
    }
}
