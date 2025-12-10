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

// Check if a point is inside or on the boundary of a rectilinear polygon
fn point_in_or_on_polygon(point: Point, polygon: &[Point]) -> bool {
    let n = polygon.len();
    
    // First check if point is on any edge
    for i in 0..n {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % n];
        
        if p1.x == p2.x && point.x == p1.x {
            // Vertical edge
            let (y_min, y_max) = if p1.y < p2.y { (p1.y, p2.y) } else { (p2.y, p1.y) };
            if point.y >= y_min && point.y <= y_max {
                return true;
            }
        } else if p1.y == p2.y && point.y == p1.y {
            // Horizontal edge
            let (x_min, x_max) = if p1.x < p2.x { (p1.x, p2.x) } else { (p2.x, p1.x) };
            if point.x >= x_min && point.x <= x_max {
                return true;
            }
        }
    }
    
    // Ray casting for interior check
    let mut inside = false;
    for i in 0..n {
        let pi = polygon[i];
        let pj = polygon[(i + 1) % n];
        
        if pi.y != pj.y {
            let (y_lo, y_hi) = if pi.y < pj.y { (pi.y, pj.y) } else { (pj.y, pi.y) };
            if point.y > y_lo && point.y <= y_hi {
                // Compute x intersection
                let x_int = pi.x + (point.y - pi.y) * (pj.x - pi.x) / (pj.y - pi.y);
                if point.x < x_int {
                    inside = !inside;
                }
            }
        }
    }
    
    inside
}

// Check if entire rectangle is inside or on polygon boundary
fn rect_inside_polygon(x1: i64, y1: i64, x2: i64, y2: i64, polygon: &[Point], xs: &[i64], ys: &[i64]) -> bool {    
    let xi1 = xs.partition_point(|&x| x < x1);
    let xi2 = xs.partition_point(|&x| x <= x2);
    let yi1 = ys.partition_point(|&y| y < y1);
    let yi2 = ys.partition_point(|&y| y <= y2);
    
    // Check corners of each cell in the compressed grid
    for xi in xi1..xi2 {
        for yi in yi1..yi2 {
            // Check the center of this cell
            let cx = (xs[xi] + xs.get(xi + 1).copied().unwrap_or(xs[xi] + 1)) / 2;
            let cy = (ys[yi] + ys.get(yi + 1).copied().unwrap_or(ys[yi] + 1)) / 2;
            
            // Ensure center is within our rectangle
            let cx = cx.max(x1).min(x2);
            let cy = cy.max(y1).min(y2);
            
            if !point_in_or_on_polygon(Point { x: cx, y: cy }, polygon) {
                return false;
            }
        }
    }
    
    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_points(input);
    if points.is_empty() {
        return Some(0);
    }
    
    // Coordinate compression: collect unique x and y values
    let mut xs: Vec<i64> = points.iter().map(|p| p.x).collect();
    let mut ys: Vec<i64> = points.iter().map(|p| p.y).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    
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
                continue; // Can't beat current best
            }
            
            if rect_inside_polygon(x_min, y_min, x_max, y_max, &points, &xs, &ys) {
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
