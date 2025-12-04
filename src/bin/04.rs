advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let bytes = input.as_bytes();
    let cols = bytes.iter().position(|&b| b == b'\n').unwrap_or(bytes.len()) + 1;
    let len = bytes.len();
    
    // Pre-compute direction offsets as flat indices
    let offsets: [isize; 8] = [
        -(cols as isize) - 1, -(cols as isize), -(cols as isize) + 1,
        -1,                                      1,
        cols as isize - 1,    cols as isize,    cols as isize + 1,
    ];
    
    let mut count = 0u64;
    
    for i in 0..len {
        if bytes[i] == b'@' {
            let mut adj = 0u8;
            for &off in &offsets {
                let ni = i as isize + off;
                if ni >= 0 && (ni as usize) < len && bytes[ni as usize] == b'@' {
                    adj += 1;
                }
            }
            if adj < 4 {
                count += 1;
            }
        }
    }
    
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let bytes = input.as_bytes();
    let cols = bytes.iter().position(|&b| b == b'\n').unwrap_or(bytes.len()) + 1;
    let len = bytes.len();
    
    let offsets: [isize; 8] = [
        -(cols as isize) - 1, -(cols as isize), -(cols as isize) + 1,
        -1,                                      1,
        cols as isize - 1,    cols as isize,    cols as isize + 1,
    ];
    
    // Use i8: -1 = not a roll, 0-8 = neighbor count
    let mut state: Vec<i8> = vec![-1; len];
    let mut stack: Vec<usize> = Vec::with_capacity(len / 4);
    
    // Initialize
    for i in 0..len {
        if bytes[i] == b'@' {
            let mut adj = 0i8;
            for &off in &offsets {
                let ni = i as isize + off;
                if ni >= 0 && (ni as usize) < len && bytes[ni as usize] == b'@' {
                    adj += 1;
                }
            }
            state[i] = adj;
            if adj < 4 {
                stack.push(i);
            }
        }
    }
    
    let mut total = 0u64;
    
    while let Some(i) = stack.pop() {
        if state[i] < 0 {
            continue;
        }
        
        state[i] = -1;
        total += 1;
        
        for &off in &offsets {
            let ni = i as isize + off;
            if ni >= 0 {
                let ni = ni as usize;
                if ni < len && state[ni] >= 0 {
                    let old = state[ni];
                    state[ni] -= 1;
                    if old == 4 {
                        stack.push(ni);
                    }
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

