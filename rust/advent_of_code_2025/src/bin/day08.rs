use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same circuit.
        }

        // Union by size.
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut sizes = Vec::new();
        for i in 0..self.parent.len() {
            if self.find(i) == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

fn part1(input: &str, num_connections: usize) -> i64 {
    // Parse junction boxes.
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect();

    // Build all edges with distances.
    let mut edges = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = points[i].distance_squared(&points[j]);
            edges.push(Reverse((dist, i, j)));
        }
    }

    // Try to connect the closest num_connections pairs.
    let mut uf = UnionFind::new(points.len());
    let mut attempts = 0;

    while attempts < num_connections && !edges.is_empty() {
        let Reverse((_, i, j)) = edges.pop().unwrap();
        uf.union(i, j); // Try to connect, even if already in same circuit.
        attempts += 1;
    }

    // Get circuit sizes and find the three largest.
    let mut sizes = uf.get_circuit_sizes();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending.

    eprintln!("Circuit sizes: {:?}", sizes);
    eprintln!("Top 3: {} × {} × {} = {}", sizes[0], sizes[1], sizes[2], sizes[0] * sizes[1] * sizes[2]);

    assert!(
        sizes.len() >= 3,
        "Expected at least 3 circuits, but found {}",
        sizes.len()
    );

    (sizes[0] * sizes[1] * sizes[2]) as i64
}

fn part2(input: &str) -> i64 {
    // Parse junction boxes.
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect();

    // Build all edges with distances.
    let mut edges = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = points[i].distance_squared(&points[j]);
            edges.push(Reverse((dist, i, j)));
        }
    }

    // Connect until all boxes are in one circuit.
    let mut uf = UnionFind::new(points.len());
    let mut found_last_connection = false;
    let mut last_connection = (0, 0);

    while !edges.is_empty() {
        let Reverse((_, i, j)) = edges.pop().unwrap();
        if uf.union(i, j) {
            // This was a successful connection (not already connected).
            last_connection = (i, j);

            // Check if we're down to 1 circuit.
            let num_circuits = uf.get_circuit_sizes().len();
            if num_circuits == 1 {
                found_last_connection = true;
                break;
            }
        }
    }

    assert!(found_last_connection, "Expected to find last connection before exiting loop!");

    let (i, j) = last_connection;
    (points[i].x as i64) * (points[j].x as i64)
}

fn main() {
    let input = include_str!("../../../../inputs/day08.txt");

    // Count number of junction boxes.
    let num_boxes = input.lines().count();

    // For the example (20 boxes), use 10 connections.
    // For the real input, use 1000 connections.
    let num_connections = if num_boxes <= 20 { 10 } else { 1000 };

    println!("{}", part1(input, num_connections));
    println!("{}", part2(input));
}
