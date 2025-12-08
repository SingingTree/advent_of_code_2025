use advent_of_code_2025::in_bounds;
use std::collections::{HashMap, HashSet};

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (row, row_vec) in grid.iter().enumerate() {
        for (col, &ch) in row_vec.iter().enumerate() {
            if ch == 'S' {
                return (row, col);
            }
        }
    }
    panic!("Start position not found in grid");
}

fn part1(input: &str) -> i64 {
    let grid = parse_grid(input);
    let (start_row, start_col) = find_start(&grid);

    let mut splits = 0;
    let mut beams: HashSet<(usize, usize)> = HashSet::new();
    beams.insert((start_row, start_col));

    while !beams.is_empty() {
        let mut next_beams = HashSet::new();

        for &(row, col) in &beams {
            // Move beam down one row.
            let new_row = row + 1;

            // Check if beam exits manifold.
            if new_row >= grid.len() {
                continue;
            }

            let ch = grid[new_row][col];

            assert_ne!(ch, 'S', "Don't expect to find S during beam traversal!");
            if ch == '.' {
                // Continue moving down.
                next_beams.insert((new_row, col));
            } else if ch == '^' {
                // Split! Beam stops, two new beams created to left and right.
                splits += 1;

                let left_col = col as i32 - 1;
                let right_col = col as i32 + 1;

                assert!(
                    in_bounds(&grid, new_row as i32, left_col),
                    "Left split out of bounds at ({}, {})",
                    new_row,
                    left_col
                );
                assert!(
                    in_bounds(&grid, new_row as i32, right_col),
                    "Right split out of bounds at ({}, {})",
                    new_row,
                    right_col
                );

                next_beams.insert((new_row, left_col as usize));
                next_beams.insert((new_row, right_col as usize));
            }
        }

        beams = next_beams;
    }

    splits
}

fn part2(input: &str) -> i64 {
    let grid = parse_grid(input);
    let (start_row, start_col) = find_start(&grid);

    let mut timelines = 0;
    // Track positions with counts - if multiple timelines reach the same position,
    // they'll behave identically from that point forward.
    let mut beams: HashMap<(usize, usize), i64> = HashMap::new();
    beams.insert((start_row, start_col), 1);

    while !beams.is_empty() {
        let mut next_beams: HashMap<(usize, usize), i64> = HashMap::new();

        for (&(row, col), &count) in &beams {
            // Move beam down one row.
            let new_row = row + 1;

            // Check if beam exits manifold.
            if new_row >= grid.len() {
                // This path is complete - count all timelines at this position.
                timelines += count;
                continue;
            }

            let ch = grid[new_row][col];

            assert_ne!(ch, 'S', "Don't expect to find S during beam traversal!");
            if ch == '.' {
                // Continue moving down.
                *next_beams.entry((new_row, col)).or_insert(0) += count;
            } else if ch == '^' {
                // Split! Create two new timelines for each timeline at this position.
                let left_col = col as i32 - 1;
                let right_col = col as i32 + 1;

                assert!(
                    in_bounds(&grid, new_row as i32, left_col),
                    "Left split out of bounds at ({}, {})",
                    new_row,
                    left_col
                );
                assert!(
                    in_bounds(&grid, new_row as i32, right_col),
                    "Right split out of bounds at ({}, {})",
                    new_row,
                    right_col
                );

                *next_beams.entry((new_row, left_col as usize)).or_insert(0) += count;
                *next_beams.entry((new_row, right_col as usize)).or_insert(0) += count;
            }
        }

        beams = next_beams;
    }

    timelines
}

fn main() {
    let input = include_str!("../../../../inputs/day07.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}
