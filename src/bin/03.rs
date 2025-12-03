advent_of_code::solution!(3);

#[derive(Debug)]
struct Bank {
    joltages: Vec<u64>,
}

#[derive(Debug)]
struct JoltagePair {
    first: u64,
    second: u64,
}

impl JoltagePair {
    fn new(first: u64, second: u64) -> Self {
        JoltagePair { first, second }
    }

    fn jolts(&self) -> u64 {
        (self.first * 10) + self.second
    }
}

impl Bank {
    fn new(line: &str) -> Self {
        let joltages = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        Bank { joltages }
    }

    fn highest_pair(&self) -> JoltagePair {
        let mut current = JoltagePair::new(0, 0);
        let mut iter = self.joltages.iter().peekable();
        while let Some(joltage) = iter.next() {
            let has_next = iter.peek().is_some();
            let test_first = JoltagePair::new(*joltage, 0);
            let test_second = JoltagePair::new(current.first, *joltage);
            if has_next && test_first.jolts() > current.jolts() {
                current = test_first;
            }
            if test_second.jolts() > current.jolts() {
                current = test_second;
            }
        }
        current
    }

    fn highest_jolts(&self) -> u64 {
        self.highest_pair().jolts()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let highest: u64 = input
        .lines()
        .map(|line| Bank::new(&line).highest_jolts())
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    Some(highest)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
