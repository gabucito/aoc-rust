use std::fmt::Display;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Matrix {
    data: Vec<bool>,
    cols: usize,
    rows: usize,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.data.len() / self.cols {
            for col in 0..self.cols {
                let char = match self.data[row * self.cols + col] {
                    true => '@',
                    false => '.',
                };
                write!(f, "{} ", char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Matrix {
    fn new(data: Vec<bool>, cols: usize, rows: usize) -> Self {
        Matrix { data, cols, rows }
    }

    fn get(&self, row: usize, col: usize) -> Option<bool> {
        if row < self.data.len() / self.cols && col < self.cols {
            Some(self.data[row * self.cols + col])
        } else {
            None
        }
    }

    fn set_index(&mut self, index: usize, value: bool) {
        self.data[index] = value;
    }

    fn set(&mut self, row: usize, col: usize, value: bool) -> Option<()> {
        if row < self.data.len() / self.cols && col < self.cols {
            self.data[row * self.cols + col] = value;
            Some(())
        } else {
            None
        }
    }

    fn get_coordinates(&self, index: usize) -> Option<(usize, usize)> {
        if index < self.data.len() {
            Some((index / self.cols, index % self.cols))
        } else {
            None
        }
    }

    fn check_above(&self, row: usize, col: usize) -> bool {
        if row == 0 {
            false
        } else {
            self.get(row - 1, col).unwrap_or(false)
        }
    }

    fn check_left(&self, row: usize, col: usize) -> bool {
        if col == 0 {
            false
        } else {
            self.get(row, col - 1).unwrap_or(false)
        }
    }

    fn check_right(&self, row: usize, col: usize) -> bool {
        if col == self.cols {
            false
        } else {
            self.get(row, col + 1).unwrap_or(false)
        }
    }

    fn check_below(&self, row: usize, col: usize) -> bool {
        if row == self.rows {
            false
        } else {
            self.get(row + 1, col).unwrap_or(false)
        }
    }

    fn check_above_left(&self, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 {
            false
        } else {
            self.get(row - 1, col - 1).unwrap_or(false)
        }
    }

    fn check_above_right(&self, row: usize, col: usize) -> bool {
        if row == 0 || col == self.cols {
            false
        } else {
            self.get(row - 1, col + 1).unwrap_or(false)
        }
    }

    fn check_below_left(&self, row: usize, col: usize) -> bool {
        if row == self.rows || col == 0 {
            false
        } else {
            self.get(row + 1, col - 1).unwrap_or(false)
        }
    }

    fn check_below_right(&self, row: usize, col: usize) -> bool {
        if row == self.rows || col == self.cols {
            false
        } else {
            self.get(row + 1, col + 1).unwrap_or(false)
        }
    }

    fn check_all(&self, index: usize) -> u8 {
        let (row, col) = self.get_coordinates(index).unwrap();
        let mut count = 0;
        if self.check_above(row, col) {
            count += 1;
        }
        if self.check_right(row, col) {
            count += 1;
        }
        if self.check_below(row, col) {
            count += 1;
        }
        if self.check_left(row, col) {
            count += 1;
        }
        if self.check_above_left(row, col) {
            count += 1;
        }
        if self.check_above_right(row, col) {
            count += 1;
        }
        if self.check_below_left(row, col) {
            count += 1;
        }
        if self.check_below_right(row, col) {
            count += 1;
        }
        count
    }
}

fn parse_input(input: &str) -> (Vec<bool>, usize, usize) {
    let mut lines = input.lines();
    let cols = lines.next().unwrap().len();
    let rows = lines.count();
    let data = input.replace("\n", "").chars().map(|c| c == '@').collect();
    (data, cols, rows)
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_input(input);
    let matrix = Matrix::new(data.0, data.1, data.2);
    let mut total = 0;
    for (index, value) in matrix.data.iter().enumerate() {
        if *value {
            let count = matrix.check_all(index);
            if count < 4 {
                total += 1;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_input(input);
    let mut matrix = Matrix::new(data.0, data.1, data.2);
    let mut total = 0;
    let mut has_neighbors = true;
    while has_neighbors {
        has_neighbors = false;
        for (index, value) in matrix.data.clone().iter().enumerate() {
            if *value {
                let count = matrix.check_all(index);
                if count < 4 {
                    total += 1;
                    matrix.set_index(index, false);
                    has_neighbors = true;
                }
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
