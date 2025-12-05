use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../../../inputs/day05.txt");

    // Split input into ranges and IDs.
    let parts: Vec<&str> = input.split("\n\n").collect();
    let ranges_str = parts[0];
    let ids_str = parts[1];

    // Parse ranges.
    let ranges: Vec<RangeInclusive<u64>> = ranges_str
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].parse().expect("Should be a number");
            let end: u64 = parts[1].parse().expect("Should be a number");
            start..=end
        })
        .collect();

    // Parse available IDs
    let ids: Vec<u64> = ids_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse().expect("Should be a number"))
        .collect();

    // Count fresh IDs
    let fresh_count = ids
        .iter()
        .filter(|&id| ranges.iter().any(|range| range.contains(id)))
        .count();

    println!("Part 1: {}", fresh_count);

    // Part 2: Count total unique IDs covered by all ranges.
    // Sort ranges by their start position so we can merge.
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();
    for range in sorted_ranges {
        if let Some(last) = merged.last_mut() {
            // If the current range overlaps or is adjacent to the last merged range...
            if range.start() <= &(last.end() + 1) {
                // ... extend the last range.
                let new_end = (*last.end()).max(*range.end());
                *last = *last.start()..=new_end;
            } else {
                // No overlap, add as new range.
                merged.push(range);
            }
        } else {
            // Handle first range.
            merged.push(range);
        }
    }

    // Count total IDs in merged ranges.
    let total_fresh: u64 = merged
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum();

    println!("Part 2: {}", total_fresh);
}
