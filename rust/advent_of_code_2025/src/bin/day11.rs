use std::collections::HashMap;

fn parse_line(line: &str) -> HashMap<String, Vec<String>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    assert!(tokens[0].ends_with(":"));

    let mut map = HashMap::new();
    map.insert(
        tokens[0].trim_matches(':').to_string(),
        tokens[1..]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    );
    map
}

fn part1(input: &str) -> usize {
    let paths = input
        .lines()
        .map(|line| parse_line(line))
        .fold(HashMap::new(), |mut acc, paths| {
            acc.extend(paths);
            acc
        });

    

    0
}

fn main() {
    let input = include_str!("../../../../inputs/day11.txt");

    println!("{}", part1(input));
}