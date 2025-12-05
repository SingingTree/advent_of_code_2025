/// Finds the maximum number that can be formed by selecting `num_digits` digits in order.
///
/// Uses a greedy algorithm: for each position in the result, picks the leftmost maximum
/// digit from a valid window. The window ensures there are enough remaining digits to
/// complete the selection.
///
/// # Algorithm
/// For each digit position (from most significant to least):
/// 1. Calculate the rightmost position we can pick from (leaving enough digits for remaining positions)
/// 2. Find the leftmost occurrence of the maximum digit in [left, right]
/// 3. Add that digit to the result and move left bound past that position
///
/// # Example
/// ```
/// // From "539142", pick 3 digits:
/// // Position 1: window [0..4], max is 9 at index 2 → pick 9, left = 3
/// // Position 2: window [3..5], max is 4 at index 4 → pick 4, left = 5
/// // Position 3: window [5..6], only 2 remains → pick 2
/// // Result: 942
/// ```
///
/// # Arguments
/// * `line` - String of digits
/// * `num_digits` - Number of digits to select
///
/// # Returns
/// The maximum number that can be formed
fn max_joltage_for_line(line: &str, num_digits: usize) -> u128 {
    let digits: Vec<u32> = line
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit"))
        .collect();
    let n = digits.len();

    let mut result = 0u128;
    let mut left = 0;

    // Work backwards through remaining digits to select
    for remaining in (1..=num_digits).rev() {
        // Calculate rightmost position we can pick from
        // (must leave `remaining - 1` digits after this one)
        let right = n - remaining;

        // Find leftmost occurrence of maximum digit in [left, right]
        let mut best_pos = left;
        let mut best_digit = digits[left];
        for i in (left + 1)..=right {
            if digits[i] > best_digit {
                best_digit = digits[i];
                best_pos = i;
            }
        }

        // Add digit to result and advance left bound
        result = result * 10 + best_digit as u128;
        left = best_pos + 1;
    }

    result
}

fn main() {
    let input = include_str!("../../../../inputs/day03.txt");

    // Part 1: Select 2 digits from each line
    let total: u128 = input
        .lines()
        .map(|line| max_joltage_for_line(line, 2))
        .sum();
    println!("{}", total);

    // Part 2: Select 12 digits from each line
    let total: u128 = input
        .lines()
        .map(|line| max_joltage_for_line(line, 12))
        .sum();
    println!("{}", total);
}
