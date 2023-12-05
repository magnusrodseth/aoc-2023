use itertools::Itertools;
use std::cmp::{max, min};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Entry {
    source_start: usize,
    source_end: usize,
    target_start: usize,
    length: usize,
}
trait Includes {
    fn includes(&self, seed: usize) -> bool;
}
impl Includes for Entry {
    /// Returns true if the given seed is included in the source range of this entry.
    fn includes(&self, seed: usize) -> bool {
        seed >= self.source_start && seed < self.source_end
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct SeedRange {
    start: usize,
    length: usize,
}
impl Includes for SeedRange {
    /// Returns true if the given seed is included in this seed range.
    fn includes(&self, seed: usize) -> bool {
        seed >= self.start && seed < self.start + self.length
    }
}

fn parse_seeds(line: &String) -> Vec<usize> {
    line.split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().expect("to be on format '79 14 55 13'"))
        .collect_vec()
}

fn parse_seed_ranges(line: &String) -> Vec<SeedRange> {
    line.split(": ")
        .last()
        .unwrap()
        .split(" ")
        .chunks(2)
        .into_iter()
        .map(|c| {
            let parts = c.collect_vec();
            SeedRange {
                start: parts[0].parse::<usize>().expect("invalid number"),
                length: parts[1].parse::<usize>().expect("invalid number"),
            }
        })
        .collect_vec()
}

fn parse_entries(lines: &Vec<String>) -> Vec<Vec<Entry>> {
    lines
        .split(|l| l.is_empty())
        .map(|chunk| {
            chunk
                .into_iter()
                // Skip the first line, which is the 'seeds: ' line
                .skip(1)
                .map(|l| {
                    let parts = l.split(" ").collect::<Vec<&str>>();

                    Entry {
                        source_start: parts[1].parse::<usize>().unwrap(),
                        source_end: parts[1].parse::<usize>().unwrap()
                            + parts[2].parse::<usize>().unwrap(),
                        target_start: parts[0].parse::<usize>().unwrap(),
                        length: parts[2].parse::<usize>().unwrap(),
                    }
                })
                .sorted()
                .collect_vec()
        })
        .collect_vec()
}

trait MergeRanges {
    fn merge_ranges(&self) -> Vec<SeedRange>;
}
impl MergeRanges for Vec<SeedRange> {
    /// Merges overlapping ranges into a new vector of ranges.
    fn merge_ranges(&self) -> Vec<SeedRange> {
        let mut merged = vec![];
        let mut current = self[0].clone();
        self.iter().for_each(|range| {
            if current.includes(range.start) {
                current.length = max(current.length, range.length + range.start - current.start);
            } else if range.start == current.start + current.length {
                current.length += range.length;
            } else {
                merged.push(current.clone());
                current = range.clone();
            }
        });
        merged.push(current);

        merged
    }
}

trait EntryMapping {
    fn map_seed(&self, seed: &usize) -> usize;
    fn map_seed_range(&self, seed_range: &SeedRange) -> Vec<SeedRange>;
}
impl EntryMapping for Vec<Entry> {
    fn map_seed(&self, seed: &usize) -> usize {
        match self.binary_search_by(|entry| entry.source_start.cmp(seed)) {
            Ok(index) => self[index].target_start,
            Err(index) => {
                if index == 0 || self[index - 1].source_end <= *seed {
                    *seed
                } else {
                    self[index - 1].target_start + seed - self[index - 1].source_start
                }
            }
        }
    }

    fn map_seed_range(&self, seed_range: &SeedRange) -> Vec<SeedRange> {
        let mut new_ranges = vec![];
        let mut new_start = seed_range.start;
        let mut remaining_length = seed_range.length;
        let mut entry_index =
            match self.binary_search_by(|entry| entry.source_start.cmp(&seed_range.start)) {
                Ok(i) => i,
                Err(i) => i,
            };

        let before_any_entry = Entry {
            source_start: 0,
            source_end: 0,
            target_start: 0,
            length: 0,
        };

        let after_any_entry = Entry {
            source_start: usize::MAX,
            source_end: usize::MAX,
            target_start: usize::MAX,
            length: 0,
        };

        while remaining_length > 0 {
            let previous = if entry_index > 0 {
                &self[entry_index - 1]
            } else {
                &before_any_entry
            };

            let next = if entry_index < self.len() {
                &self[entry_index]
            } else {
                &after_any_entry
            };

            if previous.includes(new_start) {
                new_ranges.push(SeedRange {
                    start: previous.target_start + new_start - previous.source_start,
                    length: min(remaining_length, previous.source_end - new_start),
                });
                remaining_length -= min(remaining_length, previous.source_end - new_start);
                new_start = previous.source_end;
            }

            new_ranges.push(SeedRange {
                start: new_start,
                length: min(remaining_length, next.source_start - new_start),
            });
            remaining_length -= min(remaining_length, next.source_start - new_start);
            new_start = next.source_start;

            entry_index += 1;
        }
        new_ranges
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().map(|l| l.to_string()).collect_vec();
    let seeds = parse_seeds(&lines[0]);
    let entries = parse_entries(&lines[2..].to_vec());

    let result = entries
        .iter()
        .fold(seeds, |seeds, mapping| {
            seeds
                .into_iter()
                .map(|seed| mapping.map_seed(&seed))
                .collect_vec()
        })
        .into_iter()
        .min()
        .expect("no seeds") as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().map(|l| l.to_string()).collect_vec();
    let seed_ranges = parse_seed_ranges(&lines[0]);
    let entries = parse_entries(&lines[2..].to_vec());

    Some(
        entries
            .iter()
            .fold(seed_ranges, |ranges, mapping| {
                ranges
                    .iter()
                    // Map each seed range to a vector of seed ranges
                    .flat_map(|seed_range| mapping.map_seed_range(seed_range))
                    // Filter out empty seed ranges
                    .filter(|range| range.length > 0)
                    .sorted()
                    .collect_vec()
                    // Merge overlapping seed ranges
                    .merge_ranges()
            })
            .into_iter()
            .min()
            .unwrap()
            .start as u32,
    )
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
