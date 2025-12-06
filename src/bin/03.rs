advent_of_code::solution!(3);

#[derive(Debug)]
struct Bank {
    joltages: Vec<u64>,
}

struct Sequence {
    values: Vec<Option<u64>>,
}

impl Sequence {
    fn new(length: usize) -> Self {
        let mut sequence = Sequence {
            values: vec![None; length + 1],
        };

        sequence.values[0] = Some(0);
        sequence
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

    fn highest(&self, length: usize) -> u64 {
        let mut sequence = Sequence::new(length);
        for &digit in self.joltages.iter() {
            for l in (1..=length).rev() {
                if let Some(prev) = sequence.values[l - 1] {
                    let cand = prev * 10 + digit;
                    sequence.values[l] = Some(match sequence.values[l] {
                        Some(current) => current.max(cand),
                        None => cand,
                    });
                }
            }
        }
        sequence.values[length].unwrap_or(0)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let highest: u64 = input
        .lines()
        .map(|line| Bank::new(&line).highest(2))
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    Some(highest)
}

pub fn part_two(input: &str) -> Option<u64> {
    let highest: u64 = input
        .lines()
        .map(|line| Bank::new(&line).highest(12))
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    Some(highest)
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
        assert_eq!(result, Some(3121910778619));
    }
}
