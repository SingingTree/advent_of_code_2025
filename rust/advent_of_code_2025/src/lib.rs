/// Checks if the given coordinates are within the grid bounds.
///
/// # Arguments
/// * `grid` - The 2D character grid
/// * `row` - Row coordinate (can be negative for bounds checking)
/// * `col` - Column coordinate (can be negative for bounds checking)
///
/// # Returns
/// `true` if the coordinates are valid, `false` otherwise
///
/// # Example
/// ```
/// use advent_of_code_2025::in_bounds;
/// let grid = vec![vec!['a', 'b'], vec!['c', 'd']];
/// assert!(in_bounds(&grid, 0, 0));
/// assert!(in_bounds(&grid, 1, 1));
/// assert!(!in_bounds(&grid, -1, 0));
/// assert!(!in_bounds(&grid, 2, 0));
/// ```
pub fn in_bounds(grid: &[Vec<char>], row: i32, col: i32) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    row >= 0 && row < rows && col >= 0 && col < cols
}

/// Counts neighbouring cells matching any of the target characters.
///
/// # Arguments
/// * `grid` - The 2D character grid
/// * `row` - Row coordinate of the centre cell
/// * `col` - Column coordinate of the centre cell
/// * `include_diagonals` - If `true`, counts all 8 neighbours; if `false`, only 4 orthogonal neighbours
/// * `target_chars` - Array of characters to count
///
/// # Returns
/// Number of neighbouring cells matching any character in `target_chars`
///
/// # Example
/// ```
/// use advent_of_code_2025::count_neighbors;
/// let grid = vec![
///     vec!['@', '.', '@'],
///     vec!['.', 'X', '.'],
///     vec!['@', '@', '.'],
/// ];
/// // Count '@' neighbours of centre cell with diagonals: 4
/// assert_eq!(count_neighbors(&grid, 1, 1, true, &['@']), 4);
/// // Without diagonals: 1 (only the one below)
/// assert_eq!(count_neighbors(&grid, 1, 1, false, &['@']), 1);
/// ```
pub fn count_neighbors(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    include_diagonals: bool,
    target_chars: &[char],
) -> u32 {
    let mut count = 0;

    for row_offset in -1i32..=1 {
        for col_offset in -1i32..=1 {
            // Skip the centre cell itself
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            // If not including diagonals, skip cases where both offsets are non-zero
            if !include_diagonals && row_offset != 0 && col_offset != 0 {
                continue;
            }

            let neighbour_row = row as i32 + row_offset;
            let neighbour_col = col as i32 + col_offset;

            if in_bounds(grid, neighbour_row, neighbour_col) {
                if target_chars.contains(&grid[neighbour_row as usize][neighbour_col as usize]) {
                    count += 1;
                }
            }
        }
    }
    count
}