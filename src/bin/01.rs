advent_of_code::solution!(1);

const DIAL_END: u64 = 100;
const START_POSITION: u64 = 50;

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct DialDirection {
    direction: Direction,
    quantity: u64,
}

struct Dial {
    position: u64,
}

struct Score {
    value: u64,
}

impl Score {
    fn new() -> Self {
        Score { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn increment_by(&mut self, value: u64) {
        self.value += value;
    }
}

impl Dial {
    fn new(position: u64) -> Self {
        Dial { position }
    }

    fn turn(&mut self, dial_direction: &DialDirection) -> u64 {
        let mut turns = dial_direction.quantity / DIAL_END;

        let delta: i64 = match dial_direction.direction {
            Direction::LEFT => -(dial_direction.quantity as i64) % DIAL_END as i64,
            Direction::RIGHT => dial_direction.quantity as i64 % DIAL_END as i64,
        };

        let mut new_position: i64 = self.position as i64 + delta;

        match new_position {
            101..=i64::MAX => turns += 1, // it goes over 100 +1 turn
            i64::MIN..=-1 => {
                if self.position > 0 {
                    turns += 1 // it goes negative +1 turn
                }
            }
            _ => (),
        };

        new_position = new_position % DIAL_END as i64; // in case it is 100

        // IF delta is negative
        if new_position < 0 {
            new_position += DIAL_END as i64;
        }

        self.position = new_position as u64;

        turns
    }
}

fn parse_line(line: &str) -> DialDirection {
    let direction = line.chars().next().unwrap();
    let quantity = line[1..].parse().unwrap();

    match direction {
        'R' => DialDirection {
            direction: Direction::RIGHT,
            quantity,
        },
        'L' => DialDirection {
            direction: Direction::LEFT,
            quantity,
        },
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::new(START_POSITION);
    let mut score = Score::new();
    for line in input.lines() {
        let turn = parse_line(line);
        dial.turn(&turn);
        if dial.position == 0 {
            score.increment();
        }
    }

    Some(score.value)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial::new(START_POSITION);
    let mut score = Score::new();
    for line in input.lines() {
        let turn = parse_line(line);
        let turns = dial.turn(&turn);
        score.increment_by(turns);
        if dial.position == 0 {
            score.increment();
        }
    }

    Some(score.value)
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
        assert_eq!(result, Some(6));
    }
}
