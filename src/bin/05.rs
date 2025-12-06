use std::collections::HashSet;

advent_of_code::solution!(5);

type Id = u64;

#[derive(Debug, Clone, Copy)]
struct IdRange {
    start: Id,
    end: Id,
}

impl IdRange {
    fn new(line: &str) -> Self {
        let (start, end) = line.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        IdRange { start, end }
    }

    fn is_ingredient_fresh(&self, id: Id) -> bool {
        self.start <= id && id <= self.end
    }

    fn contains_range(&self, other: &IdRange) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlaps(&self, other: &IdRange) -> bool {
        other.start <= self.end
    }

    fn merge_range(&mut self, other: IdRange) {
        self.end = self.end.max(other.end);
    }

    fn total_length(&self) -> Id {
        self.end - self.start + 1
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges_string, ingredients_string) = input.split_once("\n\n").unwrap();
    let ranges = ranges_string
        .lines()
        .map(|line| IdRange::new(line))
        .collect::<Vec<_>>();

    let fresh_ingredients = ingredients_string
        .lines()
        .map(|line| {
            let id: Id = line.parse().unwrap();
            for range in &ranges {
                if range.is_ingredient_fresh(id) {
                    return true;
                }
            }
            return false;
        })
        .filter(|&ingredient| ingredient)
        .count();

    Some(fresh_ingredients as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges_string, ingredients_string) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges_string
        .lines()
        .map(|line| IdRange::new(line))
        .collect::<Vec<_>>();

    ranges.sort_by_key(|r| r.start);
    let length = ranges.len();

    let mut merged_ranges: Vec<IdRange> = vec![ranges[0]];

    for i in 1..length {
        let last = merged_ranges.last_mut().unwrap();
        if last.overlaps(&ranges[i]) {
            last.merge_range(ranges[i]);
        } else {
            merged_ranges.push(ranges[i]);
        }
    }

    let total = merged_ranges.iter().map(|range| range.total_length()).sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
