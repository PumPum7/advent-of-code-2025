advent_of_code::solution!(5);

fn parse_and_merge(input: &str) -> Vec<(u64, u64)> {
    let ranges_str = input.split("\n\n").next().unwrap();
    let mut ranges: Vec<(u64, u64)> = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    ranges.sort_unstable_by_key(|r| r.0);

    ranges.into_iter().fold(Vec::new(), |mut merged, (s, e)| {
        if let Some(last) = merged.last_mut().filter(|l| s <= l.1 + 1) {
            last.1 = last.1.max(e);
        } else {
            merged.push((s, e));
        }
        merged
    })
}

fn is_fresh(merged: &[(u64, u64)], id: u64) -> bool {
    merged
        .binary_search_by(|&(s, e)| {
            if id < s {
                std::cmp::Ordering::Greater
            } else if id > e {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

pub fn part_one(input: &str) -> Option<u64> {
    let merged = parse_and_merge(input);
    let ids_str = input.split("\n\n").nth(1)?;

    let count = ids_str
        .lines()
        .filter(|line| is_fresh(&merged, line.parse().unwrap()))
        .count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_and_merge(input).iter().map(|(s, e)| e - s + 1).sum())
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
