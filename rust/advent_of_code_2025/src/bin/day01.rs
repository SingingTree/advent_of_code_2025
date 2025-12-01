fn parse_rotation(rotation_str: &str) -> i16 {
    let rotation_mul = match rotation_str.chars().nth(0).expect("Invalid rotation direction") {
        'R' => 1,
        'L' => -1,
        _ => panic!("invalid direction"),
    };
    let rotation_steps: i16 = rotation_str[1..].parse().expect("Invalid rotation number");
    rotation_mul * rotation_steps
}

fn update_dial(mut position: i16, mut change: i16) -> i16 {
    const NUMBER_POSITIONS: i16 = 100;

    (position + change).rem_euclid(NUMBER_POSITIONS)
}

fn update_dial_and_count_zeros(position: i16, change: i16) -> (i16, i16) {
    const NUMBER_POSITIONS: i16 = 100;

    if change == 0 {
        return (position, 0);
    }

    let new_position = (position + change).rem_euclid(NUMBER_POSITIONS);

    let zero_count = if change > 0 {
        // For right rotations, we count the number of times we 'cross' the
        // 99 -> 0 dial position to get our 0 count.
        (position + change).div_euclid(NUMBER_POSITIONS)
            - position.div_euclid(NUMBER_POSITIONS)
    } else {
        // For left rotations we need to to offset by 1 to count the number
        // of zeros we hit. For example, if we start on 0 and rotate left
        // by 1, that should not count as a 0, and by starting at position 99
        // and moving to 98 when counting rotations this is handled. A further
        // example, if we start at position 97 and rotate 97 left, that should
        // add a 0, as we end at 0. Without the the -1 offset, this wouldn't
        // be counted as we end at 0, but with the -1, this looks like a move
        // from 96 to 99, and so the cross is counted.
        (position - 1).div_euclid(NUMBER_POSITIONS)
            - (position + change - 1).div_euclid(NUMBER_POSITIONS)
    };

    (new_position, zero_count)
}

fn main() {
    let input = include_str!("../../../../inputs/day01.txt");

    let rotations: Vec<i16> = input.lines().collect::<Vec<&str>>().iter().map(
        |rotation_str| parse_rotation(rotation_str)
    ).collect();

    //let nums = lines.

    let mut position = 50;
    let mut point_to_zero_count = 0;
    for rotation in rotations.iter() {
            position = update_dial(position, *rotation);
            if position == 0 {
                point_to_zero_count += 1;
            }
    }
    println!("Part 1: {}", point_to_zero_count);

    position = 50;
    point_to_zero_count = 0;
    for rotation in rotations {
        let (new_position, zero_count) = update_dial_and_count_zeros(position, rotation);
        position = new_position;
        point_to_zero_count += zero_count;
    }

    println!("Part 2: {}", point_to_zero_count);
}
