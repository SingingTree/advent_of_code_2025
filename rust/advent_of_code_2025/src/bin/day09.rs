use rayon::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

                let width = second_tile.x.abs_diff(first_tile.x) + 1;
                let height = second_tile.y.abs_diff(first_tile.y) + 1;
                let area = height * width;
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
        // Only check vertical edges (where x coordinates are the same).
        // We can get away with this because the problem description uses
        // straight lines (either vertical or horizontal).

        assert!(
            edge.0.x == edge.1.x || edge.0.y == edge.1.y,
            "Expected all edges to be straight lines, one that wasn't: {:?}", edge
        );
        if edge.0.x == edge.1.x {
            let edge_x = edge.0.x;
            let min_y = edge.0.y.min(edge.1.y);
            let max_y = edge.0.y.max(edge.1.y);

            // Does this vertical edge span across our point's y-coordinate?
            // And is it to the right of our point?
            if min_y <= point.y && point.y < max_y && edge_x > point.x {
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
        assert!(
            edge.0.x == edge.1.x || edge.0.y == edge.1.y,
            "Expected all edges to be straight lines, one that wasn't: {:?}", edge
        );

        if edge.0.x == edge.1.x {
            // Vertical edge case.
            let min_y = edge.0.y.min(edge.1.y);
            let max_y = edge.0.y.max(edge.1.y);
            if point.x == edge.0.x && point.y >= min_y && point.y <= max_y {
                return true;
            }
        } else {
            // Horizontal edge case.
            let min_x = edge.0.x.min(edge.1.x);
            let max_x = edge.0.x.max(edge.1.x);
            if point.y == edge.0.y && point.x >= min_x && point.x <= max_x {
                return true;
            }
        }
    }
    false
}
fn point_in_polygon(
    point: &Coordinate,
    edges: &[(Coordinate, Coordinate)],
) -> bool {
    point_in_polygon_on_edge(point, edges) || point_in_polygon_ray_cast(point, edges)
}

fn polygon_edges(coordinates: &[Coordinate]) -> Vec<(Coordinate, Coordinate)> {
    // Big ole iterator fun to get
    // - all edges
    // - enure the last coord wraps to the fist for the final edge
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

fn is_rectangle_valid(
    top_left: &Coordinate,
    bottom_right: &Coordinate,
    edges: &[(Coordinate, Coordinate)],
) -> bool {
    // Quick reject: check midpoints of each edge first.
    let mid_x = (top_left.x + bottom_right.x) / 2;
    let mid_y = (top_left.y + bottom_right.y) / 2;

    if !point_in_polygon(&Coordinate { x: mid_x, y: top_left.y }, &edges) ||
        !point_in_polygon(&Coordinate { x: mid_x, y: bottom_right.y }, &edges) ||
        !point_in_polygon(&Coordinate { x: top_left.x, y: mid_y }, &edges) ||
        !point_in_polygon(&Coordinate { x: bottom_right.x, y: mid_y }, &edges) {
        return false;
    }

    // Top edge.
    for x in top_left.x..=bottom_right.x {
        if !point_in_polygon(&Coordinate { x, y: top_left.y }, &edges) {
            return false;
        }
    }

    // Right edge.
    for y in top_left.y..=bottom_right.y {
        if !point_in_polygon(&Coordinate { x: bottom_right.x, y }, &edges) {
            return false;
        }
    }

    // Bottom edge.
    for x in top_left.x..=bottom_right.x {
        if !point_in_polygon(&Coordinate { x, y: bottom_right.y }, &edges) {
            return false;
        }
    }

    // Left edge.
    for y in top_left.y..=bottom_right.y {
        if !point_in_polygon(&Coordinate { x: top_left.x, y }, &edges) {
            return false;
        }
    }

    true
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

    // Generate all pairs with their areas.
    let mut candidates: Vec<(usize, &Coordinate, &Coordinate)> = Vec::new();
    for (i, first_tile) in red_tiles.iter().enumerate() {
        for second_tile in &red_tiles[i + 1..] {
            let width = second_tile.x.abs_diff(first_tile.x) + 1;
            let height = second_tile.y.abs_diff(first_tile.y) + 1;
            candidates.push((width * height, first_tile, second_tile));
        }
    }

    // Sort by area from largest to smallest.
    candidates.sort_by(|a, b| b.0.cmp(&a.0));

    let edges = polygon_edges(&red_tiles);
    
    candidates
        .par_iter()
        .find_map_first(|(area, first_tile, second_tile)| {
            let top_left = Coordinate {
                x: first_tile.x.min(second_tile.x),
                y: first_tile.y.min(second_tile.y)
            };
            let bottom_right = Coordinate {
                x: first_tile.x.max(second_tile.x),
                y: first_tile.y.max(second_tile.y)
            };

            if is_rectangle_valid(&top_left, &bottom_right, &edges) {
                Some(*area)
            } else {
                None
            }
        })
        .expect("Should have gotten an area")
}

fn main() {
    let input = include_str!("../../../../inputs/day09.txt");

    println!("{}", part1(input));
    println!("{}", part2(input)); // 226926789 too low
}