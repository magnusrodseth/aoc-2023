use std::cmp;
use std::collections::VecDeque;

// Define types for readability and convenience
type SeedRange = (usize, usize);
type MapRange = (usize, usize, usize);
type SeedInterval = (usize, usize);
type MapInterval = (usize, usize, usize);

// Function to convert a number according to the provided map
fn convert(source_num: usize, map: &Vec<MapRange>) -> usize {
    for (dest_start, src_start, len) in map {
        if *src_start <= source_num && source_num < src_start + len {
            return (source_num - src_start) + dest_start;
        }
    }
    source_num
}

// Function to find the intersection of a seed interval and a map interval
fn intersect(seed: &SeedInterval, map: &MapInterval) -> Option<SeedInterval> {
    if seed.1 <= map.1 || seed.0 >= map.2 {
        None
    } else {
        let max_start = cmp::max(seed.0, map.1);
        let min_end = cmp::min(seed.1, map.2);
        Some((max_start, min_end))
    }
}

// Function to convert a seed interval based on a given map
fn convert_2(seed: &SeedInterval, sorted_map: &Vec<MapInterval>) -> Vec<SeedInterval> {
    let mut seeds = VecDeque::from([seed.to_owned()]);
    let mut dests = Vec::new();

    while let Some(src) = seeds.pop_front() {
        let mut intersected = false;

        for map_interval in sorted_map {
            if let Some(intersection) = intersect(&src, map_interval) {
                if src.0 < intersection.0 {
                    seeds.push_back((src.0, intersection.0 - 1));
                }
                if intersection.1 < src.1 {
                    seeds.push_back((intersection.1, src.1));
                }
                dests.push((
                    map_interval.0 + (intersection.0 - map_interval.1),
                    map_interval.0 + (intersection.1 - map_interval.1),
                ));
                intersected = true;
                break;
            }
        }
        if !intersected {
            dests.push(src);
        }
    }

    dests
}

// Parse each line of the map
fn parse_map_line(line: &str) -> MapRange {
    let nums: Vec<usize> = line
        .split_whitespace()
        .map(|w| w.parse::<usize>().expect("Map range should be a number"))
        .collect();

    // Ensure there are exactly three numbers in the line
    if nums.len() != 3 {
        panic!("Each map line should contain exactly three numbers");
    }

    (nums[0], nums[1], nums[2])
}

// Parse the seed numbers
fn parse_seeds(line: &str) -> Vec<usize> {
    line.split(": ")
        .nth(1)
        .expect("Format should be 'seeds: n1 n2 ...'")
        .split_whitespace()
        .map(|w| w.parse().expect("Seed should be a number"))
        .collect()
}

// Parse the seed ranges
fn parse_seeds_2(line: &str) -> Vec<SeedInterval> {
    line.split(": ")
        .nth(1)
        .expect("Format should be 'seeds: n1 n2 ...'")
        .split_whitespace()
        .map(|w| w.parse().expect("Seed range should be a number"))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect()
}

// Parse the input to get seeds and maps
fn parse(input: &str) -> (Vec<usize>, Vec<Vec<MapRange>>) {
    let lines: Vec<_> = input.lines().collect();
    let seeds = parse_seeds(lines[0]);
    let mut maps = Vec::new();

    let mut current_map = Vec::new();
    for line in lines.iter().skip(1) {
        if line.is_empty() {
            continue;
        }

        if line.contains("map") {
            if !current_map.is_empty() {
                maps.push(current_map);
                current_map = Vec::new();
            }
        } else {
            current_map.push(parse_map_line(line));
        }
    }

    if !current_map.is_empty() {
        maps.push(current_map);
    }

    (seeds, maps)
}

// Same as `parse` but for the second part
fn parse_2(input: &str) -> (Vec<SeedInterval>, Vec<Vec<MapInterval>>) {
    let lines: Vec<_> = input.lines().collect();
    let seeds = parse_seeds_2(lines[0]);

    let mut maps = Vec::new();
    let mut current_map = Vec::new();
    for line in lines.iter().skip(1) {
        if line.is_empty() {
            continue;
        }

        if line.contains("map") {
            if !current_map.is_empty() {
                maps.push(current_map);
                current_map = Vec::new();
            }
        } else {
            let parsed_line = parse_map_line(line);
            current_map.push((parsed_line.0, parsed_line.1, parsed_line.1 + parsed_line.2));
        }
    }

    if !current_map.is_empty() {
        maps.push(current_map);
    }

    (seeds, maps)
}

// Function to sort map ranges
fn sort_range(a: &MapRange, b: &MapRange) -> cmp::Ordering {
    a.1.cmp(&b.1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse(input);

    let min_loc = seeds
        .into_iter()
        .map(|seed| {
            maps.iter()
                .fold(seed, |prev_loc, map| convert(prev_loc, map))
        })
        .min()
        .unwrap_or(usize::MAX);

    Some(min_loc as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seeds, mut maps) = parse_2(input);

    for map in maps.iter_mut() {
        map.sort_by(sort_range);
    }

    let src_intervals = seeds;
    let result = maps
        .into_iter()
        .fold(src_intervals, |src_intervals, map| {
            src_intervals
                .into_iter()
                .flat_map(|seed_interval| convert_2(&seed_interval, &map))
                .collect()
        })
        .into_iter()
        .min_by_key(|i| i.0)
        .unwrap_or_default();

    Some(result.0 as u32)
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
