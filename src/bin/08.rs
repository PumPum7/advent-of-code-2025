advent_of_code::solution!(8);

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy)]
struct Edge {
    dist2: u64,
    a: u32,
    b: u32,
}

struct Dsu {
    parent: Vec<u32>,
    size: Vec<u32>,
    components: u32,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n as u32).collect(),
            size: vec![1; n],
            components: n as u32,
        }
    }

    fn find(&mut self, i: u32) -> u32 {
        let mut root = i;
        while root != self.parent[root as usize] {
            root = self.parent[root as usize];
        }
        let mut curr = i;
        while curr != root {
            let next = self.parent[curr as usize];
            self.parent[curr as usize] = root;
            curr = next;
        }
        root
    }

    fn union(&mut self, a: u32, b: u32) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra as usize] < self.size[rb as usize] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb as usize] = ra;
        self.size[ra as usize] += self.size[rb as usize];
        self.components -= 1;
        true
    }

    fn component_sizes(&mut self) -> Vec<u32> {
        let mut sizes = Vec::new();
        for i in 0..self.parent.len() {
            if self.find(i as u32) == i as u32 {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

fn parse_points(input: &str) -> Option<Vec<Point>> {
    let points = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next()?.trim().parse().ok()?;
            let y = parts.next()?.trim().parse().ok()?;
            let z = parts.next()?.trim().parse().ok()?;
            Some(Point { x, y, z })
        })
        .collect::<Option<Vec<_>>>()?;

    if points.is_empty() {
        None
    } else {
        Some(points)
    }
}

fn build_edges(points: &[Point]) -> Vec<Edge> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n.saturating_sub(1)) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let dx = (points[i].x - points[j].x).unsigned_abs();
            let dy = (points[i].y - points[j].y).unsigned_abs();
            let dz = (points[i].z - points[j].z).unsigned_abs();
            let dist2 = dx * dx + dy * dy + dz * dz;
            edges.push(Edge {
                dist2,
                a: i as u32,
                b: j as u32,
            });
        }
    }

    edges
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_points(input)?;
    let n = points.len();
    let mut edges = build_edges(&points);

    let connections = if n <= 20 { 10 } else { 1000 };
    
    if connections < edges.len() {
        edges.select_nth_unstable_by(connections, |a, b| {
            a.dist2
                .cmp(&b.dist2)
                .then_with(|| a.a.cmp(&b.a))
                .then_with(|| a.b.cmp(&b.b))
        });
        edges.truncate(connections);
    }

    let mut dsu = Dsu::new(n);
    for edge in edges {
        dsu.union(edge.a, edge.b);
    }

    let mut sizes = dsu.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let mut product: u128 = 1;
    for i in 0..3 {
        if let Some(&s) = sizes.get(i) {
            product *= s as u128;
        }
    }

    Some(product as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_points(input)?;
    if points.len() < 2 {
        return None;
    }

    let mut edges = build_edges(&points);
    edges.sort_unstable_by(|a, b| {
        a.dist2
            .cmp(&b.dist2)
            .then_with(|| a.a.cmp(&b.a))
            .then_with(|| a.b.cmp(&b.b))
    });

    let mut dsu = Dsu::new(points.len());
    let mut last_edge_idx = 0;

    for (i, edge) in edges.iter().enumerate() {
        if dsu.union(edge.a, edge.b) {
            if dsu.components == 1 {
                last_edge_idx = i;
                break;
            }
        }
    }

    let final_edge = edges[last_edge_idx];
    let product = (points[final_edge.a as usize].x as i128) * (points[final_edge.b as usize].x as i128);
    Some(product as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
