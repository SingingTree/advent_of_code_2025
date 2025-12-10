use std::collections::{HashMap, HashSet};

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
    indicator_lights: &Vec<bool>,
    button_wiring: &Vec<usize>
) -> Vec<bool> {
    let mut new_indicator_lights = indicator_lights.clone();
    for button in button_wiring {
        new_indicator_lights[*button] = !new_indicator_lights[*button];
    }
    new_indicator_lights
}

impl Machine {
    fn turn_on_lights_part1(&self) -> usize {
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
        presses += machine.turn_on_lights_part1();
    }
    presses
}

fn main() {
    let input = include_str!("../../../../inputs/day10.txt");

    println!("{}", part1(input));
}