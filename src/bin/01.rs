advent_of_code::solution!(1);

#[inline(always)]
fn parse_num(bytes: &[u8]) -> i64 {
    let mut n: i64 = 0;
    for &b in bytes {
        n = n * 10 + (b - b'0') as i64;
    }
    n
}

fn solve(input: &str) -> (u64, u64) {
    let mut position: i64 = 50;
    let mut count1: u64 = 0;
    let mut count2: u64 = 0;

    for line in input.as_bytes().split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }

        let is_left = line[0] == b'L';
        let distance = parse_num(&line[1..]);

        // Part 2: count all times we pass through 0
        let first = if is_left {
            if position == 0 { 100 } else { position }
        } else {
            if position == 0 { 100 } else { 100 - position }
        };

        if first <= distance {
            count2 += ((distance - first) / 100 + 1) as u64;
        }

        // Update position
        if is_left {
            position -= distance;
        } else {
            position += distance;
        }
        position = position.rem_euclid(100);

        // Part 1: count only when ending at 0
        if position == 0 {
            count1 += 1;
        }
    }

    (count1, count2)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input).0)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input).1)
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
