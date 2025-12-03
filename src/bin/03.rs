advent_of_code::solution!(3);

// Part 1: Backwards traversal to find the maximum pair of digits.
pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    for line in input.lines() {
        let mut max_suffix = 0;
        let mut max_pair = 0;
        let mut has_suffix = false;

        for &b in line.as_bytes().iter().rev() {
            if b.is_ascii_digit() {
                let d = (b - b'0') as u32;
                if has_suffix {
                    let pair = d * 10 + max_suffix;
                    if pair > max_pair {
                        max_pair = pair;
                    }
                    if d > max_suffix {
                        max_suffix = d;
                    }
                } else {
                    max_suffix = d;
                    has_suffix = true;
                }
            }
        }
        total += max_pair as u64;
    }
    Some(total)
}

// Part 2: Greedy algorithm to find the maximum
pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    for line in input.lines() {
        let digits: Vec<u8> = line.bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| b - b'0')
            .collect();

        if digits.len() < 12 {
            continue;
        }

        let mut current_num = 0;
        let mut start = 0;

        for i in 0..12 {
            let remaining_needed = 11 - i;
            let end_inclusive = digits.len() - 1 - remaining_needed;
            let slice = &digits[start..=end_inclusive];

            let mut best_d = 0;
            let mut best_offset = 0;

            for (offset, &d) in slice.iter().enumerate() {
                // 9 is the best digit so we use it
                if d == 9 {
                    best_d = 9;
                    best_offset = offset;
                    break;
                }
                if d > best_d {
                    best_d = d;
                    best_offset = offset;
                }
            }

            current_num = current_num * 10 + best_d as u64;
            start += best_offset + 1;
        }
        total += current_num;
    }
    Some(total)
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
