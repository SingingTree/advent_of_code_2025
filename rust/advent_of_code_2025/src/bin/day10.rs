use std::collections::HashSet;
use rayon::prelude::*;

struct Machine {
    // On/off indicators.
    indicator_lights: Vec<bool>,
    button_wiring: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

#[derive(Hash, Eq, PartialEq)]
struct IndicatorLightsAndCost {
    indicator_lights: Vec<bool>,
    cost: usize,
}


fn update_indicator_lights(
    indicator_lights: &[bool],
    button_wiring: &[usize],
) -> Vec<bool> {
    let mut new_indicator_lights = indicator_lights.to_vec();
    for button in button_wiring {
        new_indicator_lights[*button] = !new_indicator_lights[*button];
    }
    new_indicator_lights
}

/// Transform the augmented matrix into row echelon form using Gaussian elimination.
/// Returns the column indices of pivot columns (variables determined by the system).
/// Non-pivot columns correspond to free variables that can be chosen arbitrarily.
fn gaussian_eliminate(matrix: &mut Vec<Vec<i64>>) -> Vec<usize> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();

    // Process each column (except the last, which is the target/augmented column).
    for col in 0..cols - 1 {
        // Find a row with non-zero entry in this column to use as pivot.
        let mut found = None;
        for row in pivot_row..rows {
            if matrix[row][col] != 0 {
                found = Some(row);
                break;
            }
        }

        // If no pivot found, this is a free variable column. Skip it.
        let Some(swap_row) = found else { continue };

        // Swap the pivot row into position.
        matrix.swap(pivot_row, swap_row);
        pivot_cols.push(col);

        // Eliminate all entries below the pivot.
        // We scale rows to avoid fractions (integer arithmetic only).
        // Row[r] = Row[r] * pivot_val - Row[pivot] * row_val makes Row[r][col] = 0.
        for row in pivot_row + 1..rows {
            if matrix[row][col] != 0 {
                let pivot_val = matrix[pivot_row][col];
                let row_val = matrix[row][col];
                for c in 0..cols {
                    matrix[row][c] = matrix[row][c] * pivot_val - matrix[pivot_row][c] * row_val;
                }
            }
        }

        pivot_row += 1;
    }

    pivot_cols
}

impl Machine {
    fn turn_on_lights(&self) -> usize {
        let start_position = IndicatorLightsAndCost {
            indicator_lights: vec![false; self.indicator_lights.len()],
            cost: 0,
        };
        
        let mut seen_light_configurations: HashSet<Vec<bool>> = HashSet::new();

        let mut frontier: Vec<IndicatorLightsAndCost> = vec![start_position];
        while frontier.len() > 0 {
            let mut new_frontier: Vec<IndicatorLightsAndCost> = Vec::new();
            for indicator_lights_and_cost in frontier.iter() {
                seen_light_configurations.insert(indicator_lights_and_cost.indicator_lights.clone());
                let indicator_lights = &indicator_lights_and_cost.indicator_lights;
                let cost = indicator_lights_and_cost.cost;
                if indicator_lights == &self.indicator_lights {
                    // We've found a solution, and since we're using BFS and cost increases by 1,
                    // the first solution should be correct.
                    return cost;
                }
                for button_wiring in self.button_wiring.iter() {
                    let new_indicator_lights = update_indicator_lights(
                        indicator_lights,
                        button_wiring,
                    );
                    if !seen_light_configurations.contains(&new_indicator_lights) {
                        new_frontier.push(IndicatorLightsAndCost {
                            indicator_lights: new_indicator_lights,
                            cost: cost + 1,
                        });
                    }
                }
            }
            frontier = new_frontier;
        }

        unreachable!("Should have found a solution above");
    }

    fn build_joltage_matrix(&self) -> Vec<Vec<i64>> {
        // Matrix contains the following representation
        //        btn0  btn1  btn2 ... | target
        //   j0 [  1     0     1   ... |  3    ]
        //   j1 [  0     1     1   ... |  5    ]
        //   j2 [  1     1     0   ... |  4    ]
        let mut matrix = vec![vec![0; self.button_wiring.len() + 1]; self.joltage_requirements.len()];

        for (i, button_wiring) in self.button_wiring.iter().enumerate() {
            for &wiring in button_wiring.iter() {
                matrix[wiring][i] = 1;
            }
        }

        for (i, joltage) in self.joltage_requirements.iter().enumerate() {
            matrix[i][self.button_wiring.len()] = i64::try_from(*joltage).expect("Should be within bounds");
        }

        matrix
    }

    /// Given fixed values for free variables, solve for the pivot variables.
    /// Returns the total button presses if a valid non-negative solution exists.
    fn try_solve_buttons(
        &self,
        matrix: &Vec<Vec<i64>>,
        pivot_cols: &[usize],
        free_cols: &[usize],
        free_vals: &[i64],
    ) -> Option<usize> {
        let num_buttons = self.button_wiring.len();
        let mut solution = vec![0i64; num_buttons];

        // Fill in the free variables with the values we're trying.
        // These are the variables not determined by the row echelon form.
        for (i, &col) in free_cols.iter().enumerate() {
            solution[col] = free_vals[i];
        }

        // Solve pivot variables from bottom row up.
        // Row echelon form guarantees each pivot row has zeros below and to the left,
        // so we can solve each pivot variable using only already-solved variables.
        for (row_idx, &pivot_col) in pivot_cols.iter().enumerate().rev() {
            let row = &matrix[row_idx];
            let target = row[num_buttons]; // Last column is the target value.

            let pivot_val = row[pivot_col];

            // Sum contributions from all other variables (which are already solved).
            // Row equation: pivot_val * x_pivot + sum(row[col] * x[col]) = target.
            let mut other_sum: i64 = 0;
            for col in 0..num_buttons {
                if col != pivot_col {
                    other_sum += row[col] * solution[col];
                }
            }

            // Solve for the pivot variable: x_pivot = (target - other_sum) / pivot_val.
            let numerator = target - other_sum;
            if numerator % pivot_val != 0 {
                // Not an integer solution.
                return None;
            }
            solution[pivot_col] = numerator / pivot_val;
        }

        // Check all button presses are non-negative (can't press a button negative times).
        if solution.iter().any(|&x| x < 0) {
            return None;
        }

        Some(solution.iter().sum::<i64>() as usize)
    }

    /// Search over all possible values of free variables to find the minimum total presses.
    /// Free variables are columns without pivots - they can take any non-negative value.
    /// For each combination of free variable values, we solve for the pivot variables
    /// and check if the solution is valid (non-negative integers).
    fn solve_from_eliminated(
        &self,
        matrix: &Vec<Vec<i64>>,
        pivot_cols: &[usize],
    ) -> Option<usize> {
        let num_buttons = self.button_wiring.len();

        // Free columns are those without pivots (not determined by the system).
        let free_cols: Vec<usize> = (0..num_buttons)
            .filter(|c| !pivot_cols.contains(c))
            .collect();

        // Upper bound for free variable values. No button needs more presses than max target.
        let max_val = *self.joltage_requirements.iter().max().unwrap() as i64;
        let mut best: Option<usize> = None;

        let num_free = free_cols.len();
        let mut free_vals = vec![0i64; num_free];

        // Iterate over all combinations of free variable values from [0,0,...] to [max,max,...].
        // This is like counting in base (max_val+1).
        loop {
            if let Some(total) = self.try_solve_buttons(matrix, pivot_cols, &free_cols, &free_vals) {
                best = Some(best.map_or(total, |b| b.min(total)));
            }

            // Increment free_vals like a multi-digit counter.
            let mut carry = true;
            for i in 0..num_free {
                if carry {
                    free_vals[i] += 1;
                    if free_vals[i] > max_val {
                        free_vals[i] = 0;
                    } else {
                        carry = false;
                    }
                }
            }
            // If carry is still true, we've exhausted all combinations.
            if carry { break; }
        }

        best
    }

    fn set_joltage(&self) -> usize {
        let mut matrix = self.build_joltage_matrix();
        let pivot_cols = gaussian_eliminate(&mut matrix);

        self.solve_from_eliminated(&matrix, &pivot_cols)
            .expect("Should have a solution")
    }
}

fn parse_input_line(line: &str) -> Machine {
    let tokens: Vec<&str> = line.split(' ').collect();
    let indicator_lights: Vec<bool> = tokens[0]
        .trim_matches(['[', ']'])
        .chars()
        .map(|c| c == '#')
        .collect();
    let mut button_wiring: Vec<Vec<usize>> = Vec::new();
    for token in tokens[1..tokens.len() - 1].iter() {
        assert!(token.starts_with('('));
        assert!(token.ends_with(')'));
        let wiring: Vec<usize> = token
            .trim_matches(['(', ')'])
            .split(',')
            .map(|s| s.parse().expect("Should be a number"))
            .collect();
        button_wiring.push(wiring);
    }
    assert!(tokens[tokens.len() - 1].starts_with('{'));
    assert!(tokens[tokens.len() - 1].ends_with('}'));
    let joltage_requirements: Vec<usize> = tokens[tokens.len() - 1]
        .trim_matches(['{', '}'])
        .split(',')
        .map(|s| s.parse().expect("Should be a number"))
        .collect();
    Machine {
        indicator_lights,
        button_wiring,
        joltage_requirements,
    }
}

fn part1(input: &str) -> usize {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| parse_input_line(line))
        .collect();

    let mut presses = 0;
    for machine in machines {
        presses += machine.turn_on_lights();
    }
    presses
}

fn part2(input: &str) -> usize {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| parse_input_line(line))
        .collect();
    
    machines
        .par_iter()
        .map(|m| m.set_joltage())
        .sum()
}

fn main() {
    let input = include_str!("../../../../inputs/day10.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}