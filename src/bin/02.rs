advent_of_code::solution!(2);

fn repeated_number(num_str: &str, times: Option<usize>) -> Option<(String, usize)> {
    let len = num_str.len();
    for d in 1..=len / 2 {
        if len % d != 0 {
            continue;
        }

        let block = &num_str[0..d];
        let times = times.unwrap_or(len / d);

        if block.repeat(times) == num_str {
            return Some((block.to_string(), times));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let counter: u64 = input
        .replace("\n", "")
        .split(',')
        .map(|pair| {
            let (start, end) = pair.split_once('-').unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            let mut counter: u64 = 0;

            for num in start..=end {
                let num_str = num.to_string();
                if let Some((block, times)) = repeated_number(&num_str, Some(2)) {
                    counter += num;
                }
            }
            counter
        })
        .collect::<Vec<u64>>()
        .into_iter()
        .sum();

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let counter: u64 = input
        .replace("\n", "")
        .split(',')
        .map(|pair| {
            let (start, end) = pair.split_once('-').unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            let mut counter: u64 = 0;

            for num in start..=end {
                let num_str = num.to_string();
                if let Some((block, times)) = repeated_number(&num_str, None) {
                    counter += num;
                }
            }
            counter
        })
        .collect::<Vec<u64>>()
        .into_iter()
        .sum();

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
