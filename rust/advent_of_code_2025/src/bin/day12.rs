use std::str::Lines;

struct Shape {
    shape_num: u32,
    grid: Vec<Vec<char>>,
}

impl Shape {
    fn parse_shape<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Shape {
        let shape_num = lines
            .next()
            .expect("Should have a shape number line")
            .trim_matches(':')
            .parse::<u32>()
            .expect("Should be a number");

        let grid = lines
            .take(3)
            .map(|line| line.chars().collect())
            .collect();

        Shape { shape_num, grid }
    }

    fn min_spaces_needed(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum()
    }
}



struct Problem{
    width: usize,
    height: usize,
    required_shapes: Vec<usize>,
}

impl Problem {
    /// Parses a problem live like `49x47: 41 41 34 53 39 32`.
    fn parse_problem(line: &str) -> Problem {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        assert!(tokens[0].ends_with(':'));
        let mut dimensions = tokens[0]
            .trim_matches(':')
            .split('x')
            .map(|s| s.parse::<usize>().expect("Should be a number"));

        let required_shapes = tokens[1..]
            .iter()
            .map(|s| s.parse::<usize>().expect("Should be a number"))
            .collect();

        Problem {
            width: dimensions.next().expect("Should have a width"),
            height: dimensions.next().expect("Should have a height"),
            required_shapes,
        }
    }
}

struct BinPackingProblems {
    shapes: Vec<Shape>,
    problems: Vec<Problem>,
}


fn does_problem_have_enough_space(problem: &Problem, shapes: &[Shape]) -> bool {
    let spaces = problem.width * problem.height;
    let mut min_spaces_needed = 0;
    for (i, num_required_shapes) in problem.required_shapes.iter().enumerate() {
        min_spaces_needed += shapes[i].min_spaces_needed() * num_required_shapes;
    }

    spaces >= min_spaces_needed
}

fn parse_input(input: &str) -> BinPackingProblems {
    let mut lines = input.lines().peekable();

    let mut shapes = Vec::new();
    let mut problems = Vec::new();

    while let Some(line) = lines.peek() {
        if line.is_empty() {
            // Skip blank lines.
            lines.next();
        } else if line.ends_with(":") {
            shapes.push(Shape::parse_shape(&mut lines));
        } else {
            problems.push(Problem::parse_problem(lines.next().unwrap()));
        }
    }

    BinPackingProblems {
        shapes,
        problems,
    }
}

fn main() {
    let input = include_str!("../../../../inputs/day12.txt");

    let problems = parse_input(input);
    let mut not_enough_space_count = 0;
    for (i, problem) in problems.problems.iter().enumerate() {
        if !does_problem_have_enough_space(problem, &problems.shapes) {
            println!("Problem {} doesn't have enough space", i);
            not_enough_space_count += 1;
        }
    }
    // Part 1 is a bit of a trick -- the following is all we need.
    println!("{} problems don't have enough space, so {} do", not_enough_space_count, problems.problems.len() - not_enough_space_count)
    // No part 2 for last day.
}