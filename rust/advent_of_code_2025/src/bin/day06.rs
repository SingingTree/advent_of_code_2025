fn part1(input: &str) -> i64 {
    // Parse the worksheet into rows
    let lines: Vec<&str> = input.lines().collect();

    // Split each line by whitespace to get values
    let rows: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    // Transpose to get problems (columns)
    let num_problems = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut grand_total = 0i64;

    for col in 0..num_problems {
        // Extract values for this problem
        let mut values: Vec<&str> = Vec::new();
        for row in &rows {
            // Will assert if col is bad, but we can assume problem is well formed.
            values.push(row[col]);
        }

        // Last value is the operator
        let operator = values.last().unwrap();
        let numbers: Vec<i64> = values[..values.len()-1]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        // Calculate result based on operator
        let result: i64 = match *operator {
            "*" => numbers.iter().copied().product(),
            "+" => numbers.iter().copied().sum(),
            _ => panic!("Unknown operator: {}", operator),
        };

        grand_total += result;
    }

    grand_total
}

fn part2(input: &str) -> i64 {
    // Parse into character grid
    let lines: Vec<&str> = input.lines().collect();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            // Pad to max length in case any lines are short (probably not needed).
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    let mut grand_total = 0i64;
    let mut current_numbers: Vec<i64> = Vec::new();
    let mut current_operator: Option<char> = None;

    // Read columns from right to left
    let mut col = max_len;
    while col > 0 {
        col -= 1;

        // Read this column top to bottom, collecting digits and operator
        let mut digits = String::new();
        let mut operator: Option<char> = None;

        for row in 0..grid.len() {
            let ch = grid[row][col];
            if ch.is_ascii_digit() {
                digits.push(ch);
            } else if ch == '*' || ch == '+' {
                operator = Some(ch);
            }
        }

        // Check if this column is blank (separator)
        if digits.is_empty() && operator.is_none() {
            // Blank separator column - if we have accumulated numbers, calculate the problem
            if !current_numbers.is_empty() {
                if let Some(op) = current_operator {
                    let result: i64 = match op {
                        '*' => current_numbers.iter().copied().product(),
                        '+' => current_numbers.iter().copied().sum(),
                        _ => 0,
                    };
                    grand_total += result;
                }
                current_numbers.clear();
                current_operator = None;
            } else { 
                unreachable!("Blank separator column with no numbers to process");
            }
        } else {
            // This column has content
            if !digits.is_empty() {
                let number: i64 = digits.parse().unwrap();
                current_numbers.push(number);
            }
            if operator.is_some() {
                current_operator = operator;
            }
        }
    }

    // Handle the last problem (leftmost)
    if !current_numbers.is_empty() {
        if let Some(op) = current_operator {
            let result: i64 = match op {
                '*' => current_numbers.iter().copied().product(),
                '+' => current_numbers.iter().copied().sum(),
                _ => 0,
            };
            grand_total += result;
        }
    } else {
        unreachable!("Should have problem to process after reading leftmost column");
    }

    grand_total
}

fn main() {
    let input = include_str!("../../../../inputs/day06.txt");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}