advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    if lines.is_empty() {
        return None;
    }
    let rows = lines.len();
    let cols = lines[0].len();

    let mut start_c = 0;
    let mut start_r = 0;
    let mut found = false;
    for (r, line) in lines.iter().enumerate() {
        for (c, &b) in line.iter().enumerate() {
            if b == b'S' {
                start_r = r;
                start_c = c;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    if !found {
        return None;
    }

    let mut current_beams = vec![false; cols];
    current_beams[start_c] = true;
    let mut next_beams = vec![false; cols];

    let mut unique_splitters = 0;

    for r in start_r..rows {
        // Clear next_beams
        next_beams.fill(false);
        
        for c in 0..cols {
            if current_beams[c] {
                let byte = lines[r][c];
                match byte {
                    b'^' => {
                        unique_splitters += 1;
                        if c > 0 {
                            next_beams[c - 1] = true;
                        }
                        if c + 1 < cols {
                            next_beams[c + 1] = true;
                        }
                    }
                    _ => {
                        // Passes through to the same column in the next row
                        next_beams[c] = true;
                    }
                }
            }
        }
        
        // Swap buffers
        std::mem::swap(&mut current_beams, &mut next_beams);
        
        // If no beams left, break early 
        if !current_beams.iter().any(|&x| x) {
            break;
        }
    }

    Some(unique_splitters)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    if lines.is_empty() {
        return None;
    }
    let rows = lines.len();
    let cols = lines[0].len();

    let mut start_c = 0;
    let mut start_r = 0;
    let mut found = false;
    for (r, line) in lines.iter().enumerate() {
        for (c, &b) in line.iter().enumerate() {
            if b == b'S' {
                start_r = r;
                start_c = c;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    if !found {
        return None;
    }

    let mut current_counts = vec![0u64; cols];
    current_counts[start_c] = 1;
    let mut next_counts = vec![0u64; cols];

    let mut total_timelines = 0;

    for r in start_r..rows {
        next_counts.fill(0);
        
        for c in 0..cols {
            let count = current_counts[c];
            if count == 0 {
                continue;
            }

            let byte = lines[r][c];
            match byte {
                b'^' => {
                    // Left
                    if c > 0 {
                        next_counts[c - 1] += count;
                    } else {
                        // Exited left
                        total_timelines += count;
                    }
                    // Right
                    if c + 1 < cols {
                        next_counts[c + 1] += count;
                    } else {
                        // Exited right
                        total_timelines += count;
                    }
                }
                _ => {
                    // Down
                    next_counts[c] += count;
                }
            }
        }
        
        std::mem::swap(&mut current_counts, &mut next_counts);
    }

    for &count in &current_counts {
        total_timelines += count;
    }

    Some(total_timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
