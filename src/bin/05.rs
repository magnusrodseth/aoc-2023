#[derive(Debug)]
struct EntryData {
    source_range_start: u32,
    destination_range_start: u32,
    range_length: u32,
}

fn parse_entries(input: &str) -> Vec<Vec<EntryData>> {
    let lines = input
        .lines()
        // Remove the two first lines, for seeds
        .skip(2)
        .collect::<Vec<&str>>();

    let mut entries: Vec<Vec<EntryData>> = Vec::new();
    let mut current_entry_data: Vec<EntryData> = Vec::new();

    for line in &lines {
        if line.is_empty() {
            if !current_entry_data.is_empty() {
                entries.push(current_entry_data);
                current_entry_data = Vec::new();
            }
            continue;
        }

        if line.ends_with("map:") {
            if !current_entry_data.is_empty() {
                entries.push(current_entry_data);
            }
            current_entry_data = Vec::new();
        } else {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            let data = EntryData {
                source_range_start: numbers[1],
                destination_range_start: numbers[0],
                range_length: numbers[2],
            };

            current_entry_data.push(data);
        }
    }

    if !current_entry_data.is_empty() {
        entries.push(current_entry_data);
    }

    entries
}

fn convert_number(number: u32, entry: &[EntryData]) -> u32 {
    for entry in entry {
        let is_within_range = (number >= entry.source_range_start)
            && (number < entry.source_range_start + entry.range_length);

        if is_within_range {
            return entry.destination_range_start + (number - entry.source_range_start);
        }
    }

    number
}

pub fn part_one(input: &str) -> Option<u32> {
    // Find seeds to be planted
    let seeds_to_plant = input
        .lines()
        .nth(0)
        .expect("should have one line")
        .split("seeds: ")
        .nth(1)
        .expect("should have seeds")
        .split(" ")
        .map(|s| s.parse::<u32>().expect("should be a number"))
        .collect::<Vec<u32>>();

    let entries = parse_entries(input);

    let mut lowest_location = u32::MAX;

    for seed in seeds_to_plant {
        let mut next_number = seed;

        // Convert the seed number through each map
        for entry in &entries {
            next_number = convert_number(next_number, entry);
        }

        // Track the lowest location number
        if next_number < lowest_location {
            lowest_location = next_number;
        }
    }

    if lowest_location == u32::MAX {
        return None;
    }

    Some(lowest_location)
}

#[derive(Debug)]
struct SeedRange {
    source: u32,
    range_length: u32,
}

pub fn part_two(input: &str) -> Option<u32> {
    // Find range of seeds to be planted
    let seeds_to_plant = input
        .lines()
        .nth(0)
        .expect("should have one line")
        .split("seeds: ")
        .nth(1)
        .expect("should have seeds")
        .split(" ")
        .collect::<Vec<&str>>()
        // Convert "79 14 55 13" to chunks of two, i.e. ["79", "14"], ["55", "13"] etc.
        .chunks(2)
        .map(|chunk| SeedRange {
            source: chunk[0].parse::<u32>().expect("should be a number"),
            range_length: chunk[1].parse::<u32>().expect("should be a number"),
        })
        .collect::<Vec<SeedRange>>()
        // Map the seed range to the range of numbers that will be planted
        .iter()
        .map(|seed_range| {
            let mut range = Vec::new();
            for i in 0..seed_range.range_length {
                range.push(seed_range.source + i);
            }
            range
        })
        .collect::<Vec<Vec<u32>>>()
        // Flatten the list of ranges into a single list of numbers
        .iter()
        .flatten()
        .map(|&num| num)
        .collect::<Vec<u32>>();

    let entries = parse_entries(input);

    let mut lowest_location = u32::MAX;

    for seed in seeds_to_plant {
        let mut next_number = seed;

        // Convert the seed number through each map
        for entry in &entries {
            next_number = convert_number(next_number, entry);
        }

        // Track the lowest location number
        if next_number < lowest_location {
            lowest_location = next_number;
        }
    }

    if lowest_location == u32::MAX {
        return None;
    }

    Some(lowest_location)
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(46));
    }
}
