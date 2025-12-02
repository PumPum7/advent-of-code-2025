advent_of_code::solution!(2);

// Part 1: Find IDs where a sequence is repeated exactly twice (e.g., 55, 6464, 123123).
fn find_invalid_ids_in_range_p1(start: u64, end: u64) -> u64 {
    let mut sum = 0;

    for half_len in 1..=10 {
        let multiplier = 10_u64.pow(half_len) + 1;

        let min_half = if half_len == 1 {
            1
        } else {
            10_u64.pow(half_len - 1)
        };
        let max_half = 10_u64.pow(half_len) - 1;

        let lo = start.div_ceil(multiplier).max(min_half);
        let hi = (end / multiplier).min(max_half);

        if lo <= hi {
            let count = hi - lo + 1;
            let half_sum = count * (lo + hi) / 2;
            sum += half_sum * multiplier;
        }
    }

    sum
}

// Check if a k-digit pattern is "primitive" (not a repetition of a shorter pattern)
fn is_primitive(pattern: u64, k: u32) -> bool {
    for d in 1..k {
        if k % d != 0 {
            continue;
        }
        // Check if pattern == (first d digits) repeated k/d times
        let sub = pattern / 10_u64.pow(k - d);
        let reps = k / d;
        let base = 10_u64.pow(d);

        let mut mult = 0u64;
        let mut pow = 1u64;
        for _ in 0..reps {
            mult += pow;
            pow *= base;
        }

        if sub * mult == pattern {
            return false;
        }
    }
    true
}

// Part 2: Sum invalid IDs in range (pattern repeated at least twice, only counting primitive patterns)
fn find_invalid_ids_in_range_p2(start: u64, end: u64) -> u64 {
    let mut sum = 0u64;
    let max_digits = 11u32;

    for pattern_len in 1..=max_digits / 2 {
        let min_pattern = if pattern_len == 1 {
            1
        } else {
            10_u64.pow(pattern_len - 1)
        };
        let max_pattern = 10_u64.pow(pattern_len) - 1;

        for repetitions in 2..=max_digits / pattern_len {
            let base = 10_u64.pow(pattern_len);
            let mut multiplier = 0u64;
            let mut pow = 1u64;
            for _ in 0..repetitions {
                multiplier += pow;
                pow *= base;
            }

            let lo = start.div_ceil(multiplier).max(min_pattern);
            let hi = (end / multiplier).min(max_pattern);

            if lo <= hi {
                if pattern_len == 1 {
                    // All 1-digit patterns are primitive - use arithmetic formula
                    let count = hi - lo + 1;
                    sum += count * (lo + hi) / 2 * multiplier;
                } else {
                    // For larger patterns, iterate (range is typically small)
                    for pattern in lo..=hi {
                        if is_primitive(pattern, pattern_len) {
                            sum += pattern * multiplier;
                        }
                    }
                }
            }
        }
    }

    sum
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|range| {
            let mut parts = range.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);
    let sum: u64 = ranges
        .iter()
        .map(|&(start, end)| find_invalid_ids_in_range_p1(start, end))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);
    let sum: u64 = ranges
        .iter()
        .map(|&(start, end)| find_invalid_ids_in_range_p2(start, end))
        .sum();
    Some(sum)
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
