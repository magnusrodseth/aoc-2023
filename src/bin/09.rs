struct Sequence {
    values: Vec<i32>,
}

impl Sequence {
    fn is_all_zeroes(&self) -> bool {
        self.values.iter().all(|&x| x == 0)
    }

    fn add_next(&mut self) {
        if self.is_all_zeroes() {
            self.values.push(0);
            return;
        }

        let mut sub_sequence = Sequence {
            values: self
                .values
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect(),
        };
        sub_sequence.add_next();

        let result = self.values.last().unwrap() + sub_sequence.values.last().unwrap_or(&0);
        self.values.push(result);
    }

    fn add_previous(&mut self) {
        if self.is_all_zeroes() {
            self.values.insert(0, 0);
            return;
        }

        let mut sub_sequence = Sequence {
            values: self
                .values
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect(),
        };
        sub_sequence.add_previous();

        let result = self.values[0] - sub_sequence.values[0];
        self.values.insert(0, result);
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let histories = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|digit| digit.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut sum = 0;

    for history in histories {
        let mut sequence = Sequence { values: history };
        sequence.add_next();
        sum += sequence.values.last().unwrap_or(&0);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let histories = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|digit| digit.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut sum = 0;

    for history in histories {
        let mut sequence = Sequence { values: history };
        sequence.add_previous();
        sum += sequence.values[0];
    }

    Some(sum)
}

advent_of_code::main!(9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(2));
    }
}
