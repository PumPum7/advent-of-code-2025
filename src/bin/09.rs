advent_of_code::solution!(9);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            let mut parts = line.split(',');
            let x = parts.next()?.trim().parse().ok()?;
            let y = parts.next()?.trim().parse().ok()?;
            Some(Point { x, y })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_points(input);
    
    let mut max_area = 0u64;
    
    // Try all pairs of red tiles as potential opposite corners
    // The other two corners don't need to be red - only the two opposite corners must be red
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            
            // For two points to form opposite corners
            // The rectangle includes both endpoints, so we add 1 to width and height
            let width = (p2.x - p1.x).unsigned_abs() + 1;
            let height = (p2.y - p1.y).unsigned_abs() + 1;
            
            // Count rectangles (including thin rectangles/lines, but not single points)
            if width * height > 1 {
                let area = width * height;
                max_area = max_area.max(area);
            }
        }
    }
    
    Some(max_area)
}

// Check if a point is inside a rectilinear polygon using ray casting
fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    
    for i in 0..n {
        let pi = polygon[i];
        let pj = polygon[(i + 1) % n];
        
        if pi.y != pj.y {
            let (y_lo, y_hi) = if pi.y < pj.y { (pi.y, pj.y) } else { (pj.y, pi.y) };
            if point.y > y_lo && point.y <= y_hi {
                let x_int = pi.x + (point.y - pi.y) * (pj.x - pi.x) / (pj.y - pi.y);
                if point.x < x_int {
                    inside = !inside;
                }
            }
        }
    }
    
    inside
}

pub fn part_two(input: &str) -> Option<u64> {
    use std::collections::HashMap;
    
    let points = parse_points(input);
    if points.is_empty() {
        return Some(0);
    }
    
    // Coordinate compression
    let mut xs: Vec<i64> = points.iter().map(|p| p.x).collect();
    let mut ys: Vec<i64> = points.iter().map(|p| p.y).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    
    let x_to_idx: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_to_idx: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();
    
    let nx = xs.len();
    let ny = ys.len();
    
    // Precompute which cells are inside the polygon
    // A cell (xi, yi) represents the region from xs[xi] to xs[xi+1]-1 and ys[yi] to ys[yi+1]-1
    // We check if the cell center is inside
    let mut inside = vec![vec![false; ny]; nx];
    
    for xi in 0..nx {
        for yi in 0..ny {
            // Use cell corner + small offset to check interior
            let test_x = xs[xi];
            let test_y = ys[yi];
            // A point on the boundary should count as inside
            // Check a point just inside the cell
            let next_x = xs.get(xi + 1).copied().unwrap_or(xs[xi] + 2);
            let next_y = ys.get(yi + 1).copied().unwrap_or(ys[yi] + 2);
            let mid_x = (xs[xi] + next_x) / 2;
            let mid_y = (ys[yi] + next_y) / 2;
            
            inside[xi][yi] = point_in_polygon(Point { x: mid_x, y: mid_y }, &points);
        }
    }
    
    // Build 2D prefix sum of "outside" cells for fast rectangle queries
    // prefix[xi][yi] = number of outside cells in [0..xi) x [0..yi)
    let mut prefix = vec![vec![0i32; ny + 1]; nx + 1];
    for xi in 0..nx {
        for yi in 0..ny {
            let val = if inside[xi][yi] { 0 } else { 1 };
            prefix[xi + 1][yi + 1] = prefix[xi][yi + 1] + prefix[xi + 1][yi] - prefix[xi][yi] + val;
        }
    }
    
    // Query: count outside cells in [xi1..xi2) x [yi1..yi2)
    let count_outside = |xi1: usize, yi1: usize, xi2: usize, yi2: usize| -> i32 {
        prefix[xi2][yi2] - prefix[xi1][yi2] - prefix[xi2][yi1] + prefix[xi1][yi1]
    };
    
    let mut max_area = 0u64;
    
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            
            let x_min = p1.x.min(p2.x);
            let x_max = p1.x.max(p2.x);
            let y_min = p1.y.min(p2.y);
            let y_max = p1.y.max(p2.y);
            
            let width = (x_max - x_min) as u64 + 1;
            let height = (y_max - y_min) as u64 + 1;
            
            if width * height <= max_area {
                continue;
            }
            
            // Get compressed indices
            let xi1 = x_to_idx[&x_min];
            let xi2 = x_to_idx[&x_max];
            let yi1 = y_to_idx[&y_min];
            let yi2 = y_to_idx[&y_max];
            
            // Check if any cell in range is outside
            if count_outside(xi1, yi1, xi2 + 1, yi2 + 1) == 0 {
                max_area = width * height;
            }
        }
    }
    
    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
