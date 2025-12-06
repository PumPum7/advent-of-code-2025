advent_of_code::solution!(6);

struct Grid {
    data: Vec<Vec<u8>>,
    ops: Vec<u8>,
    separators: Vec<bool>,
    max_width: usize,
}

impl Grid {
    fn from_input(input: &str) -> Option<Self> {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return None;
        }

        let operator_row = lines.last().unwrap();
        let data_rows = &lines[..lines.len() - 1];

        let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let data: Vec<Vec<u8>> = data_rows
            .iter()
            .map(|row| {
                let mut bytes: Vec<u8> = row.as_bytes().to_vec();
                bytes.resize(max_width, b' ');
                bytes
            })
            .collect();
        let ops: Vec<u8> = {
            let mut bytes: Vec<u8> = operator_row.as_bytes().to_vec();
            bytes.resize(max_width, b' ');
            bytes
        };

        let separators: Vec<bool> = (0..max_width)
            .map(|col| data.iter().all(|row| row[col] == b' '))
            .collect();

        Some(Self {
            data,
            ops,
            separators,
            max_width,
        })
    }

    fn apply_op(op: u8, numbers: &[u64]) -> u64 {
        match op {
            b'+' => numbers.iter().sum(),
            b'*' => numbers.iter().product(),
            _ => 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_input(input)?;
    let data_rows = grid.data.len();

    let mut total: u64 = 0;
    let mut col = 0;

    while col < grid.max_width {
        if grid.separators[col] {
            col += 1;
            continue;
        }

        let start_col = col;
        while col < grid.max_width && !grid.separators[col] {
            col += 1;
        }
        let end_col = col;

        let mut numbers: Vec<u64> = Vec::with_capacity(data_rows);
        for row in &grid.data {
            let slice = &row[start_col..end_col];
            let start = slice.iter().position(|&b| b != b' ').unwrap_or(0);
            let end = slice.iter().rposition(|&b| b != b' ').map(|i| i + 1).unwrap_or(0);
            
            if start < end {
                let mut num = 0u64;
                let mut found_digit = false;
                for &b in &slice[start..end] {
                    if b.is_ascii_digit() {
                        num = num * 10 + (b - b'0') as u64;
                        found_digit = true;
                    }
                }
                if found_digit {
                    numbers.push(num);
                }
            }
        }

        let op = grid.ops[start_col..end_col]
            .iter()
            .find(|&&b| b == b'+' || b == b'*')
            .copied()
            .unwrap_or(b'+');

        total += Grid::apply_op(op, &numbers);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::from_input(input)?;

    let mut total: u64 = 0;
    let mut col = grid.max_width;

    while col > 0 {
        col -= 1;
        
        if grid.separators[col] {
            continue;
        }

        if grid.ops[col] != b'+' && grid.ops[col] != b'*' {
            continue;
        }

        let op = grid.ops[col];
        
        let problem_start = col;
        let mut problem_end = col + 1;
        while problem_end < grid.max_width && !grid.separators[problem_end] {
            problem_end += 1;
        }

        let mut numbers: Vec<u64> = Vec::with_capacity(problem_end - problem_start);
        for c in problem_start..problem_end {
            let mut num = 0u64;
            let mut has_digit = false;
            for row in &grid.data {
                let b = row[c];
                if b.is_ascii_digit() {
                    num = num * 10 + (b - b'0') as u64;
                    has_digit = true;
                }
            }
            
            if has_digit {
                numbers.push(num);
            }
        }

        total += Grid::apply_op(op, &numbers);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
