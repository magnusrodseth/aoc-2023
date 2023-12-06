fn ways_to_win(time: u64, record_distance: u64) -> u64 {
    let mut ways_to_win = 0;

    for hold_time in 0..time {
        let travel_time = time - hold_time;
        let distance_travelled = hold_time * travel_time;
        if distance_travelled > record_distance {
            ways_to_win += 1;
        }
    }

    ways_to_win
}

pub fn part_one(input: &str) -> Option<u64> {
    let times = input
        .lines()
        .nth(0)
        .expect("should have first line")
        .split("Time: ")
        .nth(1)
        .expect("should have times")
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("should be a number"))
        .collect::<Vec<u64>>();

    let distances = input
        .lines()
        .nth(1)
        .expect("should have second line")
        .split("Distance: ")
        .nth(1)
        .expect("should have distances")
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("should be a number"))
        .collect::<Vec<u64>>();

    assert_eq!(times.len(), distances.len());

    let mut total_ways = 1;

    for (&time, &record_distance) in times.iter().zip(distances.iter()) {
        total_ways *= ways_to_win(time, record_distance);
    }

    Some(total_ways)
}

pub fn part_two(input: &str) -> Option<u64> {
    let times = input
        .lines()
        .nth(0)
        .expect("should have first line")
        .split("Time: ")
        .nth(1)
        .expect("should have times")
        // Merge strings together from "7  15   30" to "71530"
        .split_whitespace()
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc
        })
        // Parse the merged string as one big number
        .parse::<u64>()
        .expect("should be a number");

    let distances = input
        .lines()
        .nth(1)
        .expect("should have second line")
        .split("Distance: ")
        .nth(1)
        .expect("should have distances")
        // Merge strings together from "7  15   30" to "71530"
        .split_whitespace()
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc
        })
        // Parse the merged string as one big number
        .parse::<u64>()
        .expect("should be a number");

    Some(ways_to_win(times, distances))
}

advent_of_code::main!(6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(71503));
    }
}
