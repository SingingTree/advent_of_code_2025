use std::collections::VecDeque;
use advent_of_code_2025::{count_neighbors, in_bounds};

/// Counts all '@' cells that have fewer than 4 '@' neighbors (including diagonals).
///
/// A cell is "accessible" if it has fewer than 4 neighboring '@' cells in the 8
/// surrounding positions (orthogonal + diagonal).
fn part1(map: &str) {
    let grid: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != '@' {
                continue;
            }

            let neighbors = count_neighbors(&grid, row, col, true, &['@']);

            // Cell is accessible if it has fewer than 4 '@' neighbors
            if neighbors < 4 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

/// Removes all accessible '@' cells using BFS with cascading accessibility checks.
///
/// Starting from initially accessible cells (those with <4 '@' neighbors), removes them
/// one by one. After each removal, checks all 8 neighbors to see if they've become
/// accessible (since removing a cell reduces their neighbor count).
///
/// This creates a cascading effect where removing one cell can make others accessible.
fn part2(map: &str) {
    let mut grid: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut removed = 0;
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // Initial pass: find all accessible '@' cells (those with <4 '@' neighbors)
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '@' && count_neighbors(&grid, row, col, true, &['@']) < 4 {
                queue.push_back((row, col));
            }
        }
    }

    // BFS: process accessible cells and check neighbors after removal
    while let Some((row, col)) = queue.pop_front() {
        // Might have been removed already or no longer accessible
        if grid[row][col] != '@' || count_neighbors(&grid, row, col, true, &['@']) >= 4 {
            continue;
        }

        // Remove this cell
        grid[row][col] = '.';
        removed += 1;

        // Check all 8 neighbors - they might now be accessible (fewer neighbors after removal)
        for row_offset in -1i32..=1 {
            for col_offset in -1i32..=1 {
                if row_offset == 0 && col_offset == 0 {
                    // Skip self -- not a neighbor
                    continue;
                }
                let neighbour_row = row as i32 + row_offset;
                let neighbour_col = col as i32 + col_offset;
                if in_bounds(&grid, neighbour_row, neighbour_col) {
                    let nr = neighbour_row as usize;
                    let nc = neighbour_col as usize;
                    if grid[nr][nc] == '@' {
                        // Add to queue - it might now be accessible
                        queue.push_back((nr, nc));
                    }
                }
            }
        }
    }

    println!("{}", removed);
}

fn main() {
    let input = include_str!("../../../../inputs/day04.txt");

    part1(input);
    part2(input);
}