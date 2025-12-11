use std::collections::HashMap;

fn parse_line(line: &str) -> HashMap<String, Vec<String>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    assert!(tokens[0].ends_with(":"));

    let mut map = HashMap::new();
    map.insert(
        tokens[0].trim_matches(':').to_string(),
        tokens[1..].iter().map(|s| s.to_string()).collect(),
    );
    map
}

fn count_paths(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if node == "out" {
        return 1;
    }

    if let Some(&count) = memo.get(node) {
        return count;
    }

    let count = match graph.get(node) {
        Some(children) => children
            .iter()
            .map(|child| count_paths(child, graph, memo))
            .sum(),
        None => 0,
    };

    memo.insert(node.to_string(), count);
    count
}

fn part1(input: &str) -> usize {
    let graph =
        input
            .lines()
            .map(|line| parse_line(line))
            .fold(HashMap::new(), |mut acc, paths| {
                acc.extend(paths);
                acc
            });

    let mut memo = HashMap::new();
    count_paths("you", &graph, &mut memo)
}

fn count_paths_with_required_nodes(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    visited_dac: bool,
    visited_fft: bool,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    let visited_dac = visited_dac || node == "dac";
    let visited_fft = visited_fft || node == "fft";

    if node == "out" {
        return if visited_dac && visited_fft { 1 } else { 0 };
    }

    let key = (node.to_string(), visited_dac, visited_fft);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    let count = match graph.get(node) {
        Some(children) => children
            .iter()
            .map(|child| {
                count_paths_with_required_nodes(child, graph, visited_dac, visited_fft, memo)
            })
            .sum(),
        None => 0,
    };

    memo.insert(key, count);
    count
}

fn part2(input: &str) -> usize {
    let graph =
        input
            .lines()
            .map(|line| parse_line(line))
            .fold(HashMap::new(), |mut acc, paths| {
                acc.extend(paths);
                acc
            });

    let mut memo = HashMap::new();
    count_paths_with_required_nodes("svr", &graph, false, false, &mut memo)
}

fn main() {
    let input = include_str!("../../../../inputs/day11.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}
