use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq)]
#[derive(Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn part1(input: &str) -> usize {
    let red_tiles: Vec<Coordinate> = input
        .lines()
        .map(|line| {
            let digits: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().expect("Should be a number"))
                .collect();
            assert_eq!(digits.len(), 2);
            Coordinate { x: digits[0], y: digits[1] }
        }).collect();

        let mut max_area = 0;
        for (i, first_tile) in red_tiles.iter().enumerate() {
            for second_tile in &red_tiles[i + 1..] {
                let height = second_tile.x.abs_diff(first_tile.x) + 1;
                let width = second_tile.y.abs_diff(first_tile.y) + 1;
                let area = height * width;
                //println!("{},{} * {},{} = {}", first_tile.x, first_tile.y, second_tile.x, second_tile.y, area,);
                if area > max_area {
                    max_area = area;
                }
            }
        }

    max_area
}

fn point_in_polygon_ray_cast(
    point: &Coordinate,
    edges: &[(Coordinate, Coordinate)],
) -> bool {
    let mut count = 0;
    for edge in edges {
        if edge.0.y <= point.y && edge.1.y > point.y || edge.1.y <= point.y && edge.0.y > point.y {
            if edge.0.x <= point.x && edge.1.x > point.x || edge.1.x <= point.x && edge.0.x > point.x {
                count += 1;
            }
        }
    }
    count % 2 == 1
}

fn point_in_polygon_on_edge(
    point: &Coordinate,
    edges: &[(Coordinate, Coordinate)],
) -> bool {
    for edge in edges {
        if edge.0.x == point.x || edge.1.x == point.x {
            // Point is on an x-aligned edge.
            return true;
        }
        if edge.0.y == point.y || edge.1.y == point.y {
            // Point is on a y-aligned edge.
            return true;
        }
    }
    false
}
fn point_in_polygon(
    point: &Coordinate,
    edges: &[(Coordinate, Coordinate)],
) -> bool {
    point_in_polygon_ray_cast(point, edges) || point_in_polygon_on_edge(point, edges)
}

fn polygon_edges(coordinates: &[Coordinate]) -> Vec<(Coordinate, Coordinate)> {
    // Big ole iterator fun to get
    // - all edges
    // - enure the last coord wraps to the frist for the final edge
    coordinates
        .iter()
        // Chain into the first item again to get last egde.
        .chain(coordinates.iter().take(1))
        .copied()
        // Collect as windows needs a Vec.
        .collect::<Vec<Coordinate>>()
        .windows(2)
        .map(|pair| (pair[0], pair[1]))
        .collect()
}

struct PointChecker {
    point_cache: HashMap<Coordinate, bool>,
}

impl PointChecker {
    fn new() -> Self {
        Self { point_cache: HashMap::new() }
    }

    fn check(&mut self, point: &Coordinate, edges: &[(Coordinate, Coordinate)]) -> bool {
        if let Some(cached) = self.point_cache.get(point) {
            return *cached;
        }
        let result = point_in_polygon(point, edges);
        self.point_cache.insert(*point, result);
        result
    }
}

fn part2(input: &str) -> usize {
    let red_tiles: Vec<Coordinate> = input
        .lines()
        .map(|line| {
            let digits: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().expect("Should be a number"))
                .collect();
            assert_eq!(digits.len(), 2);
            Coordinate { x: digits[0], y: digits[1] }
        }).collect();

    let edges = polygon_edges(&red_tiles);

    let mut checker = PointChecker::new();

    let mut max_area = 0;
    for (i, first_tile) in red_tiles.iter().enumerate() {
        for second_tile in &red_tiles[i + 1..] {
            let mut area = 0;
            let top_left = Coordinate { x: first_tile.x.min(second_tile.x), y: first_tile.y.min(second_tile.y) };
            let bottom_right = Coordinate { x: first_tile.x.max(second_tile.x), y: first_tile.y.max(second_tile.y) };
            for x in top_left.x..=bottom_right.x {
                for y in top_left.y..=bottom_right.y {
                    if checker.check(&Coordinate { x, y }, &edges) {
                        area += 1;
                    }
                }
            }
            if area > max_area {
                max_area = area;
            }
        }
        println!("Done iteration {i}")
    }

    max_area
}

fn main() {
    let input = include_str!("../../../../inputs/day09.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}