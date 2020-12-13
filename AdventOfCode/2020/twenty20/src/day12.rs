#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<(char, usize)> {
    input
        .lines()
        .map(|l| (l[..1].chars().next().unwrap(), l[1..].parse().unwrap()))
        .collect()
}

fn get_position_after_move(position: (i32, i32), direction: char, distance: i32) -> (i32, i32) {
    match direction {
        'N' => return (position.0, position.1 + distance),
        'S' => return (position.0, position.1 - distance),
        'E' => return (position.0 + distance, position.1),
        'W' => return (position.0 - distance, position.1),
        _ => return position,
    }
}

fn get_direction_after_rotation(direction: char, rotation: usize) -> char {
    let directions = ['N', 'E', 'S', 'W'];
    let current_direction_index = directions.iter().position(|&r| r == direction).unwrap();

    let number_of_rotations = rotation / 90;
    let result = directions[(current_direction_index + number_of_rotations) % 4];
    println!(
        "{} {} {}",
        current_direction_index, number_of_rotations, result
    );
    return result;
}

fn get_position_after_move_waypoint(
    waypoint_position: (i32, i32),
    position: (i32, i32),
    times: usize,
) -> (i32, i32) {
    return (
        position.0 + waypoint_position.0 * times as i32,
        position.1 + waypoint_position.1 * times as i32,
    );
}

#[aoc(day12, part1)]
fn solve_part1(input: &[(char, usize)]) -> i32 {
    println!("{:?}", input);

    let mut direction = 'E';
    let mut position = (0, 0);

    input.iter().for_each(|instruction| match instruction.0 {
        'N' => {
            position = get_position_after_move(position, 'N', instruction.1 as i32);
        }
        'S' => {
            position = get_position_after_move(position, 'S', instruction.1 as i32);
        }
        'E' => {
            position = get_position_after_move(position, 'E', instruction.1 as i32);
        }
        'W' => {
            position = get_position_after_move(position, 'W', instruction.1 as i32);
        }
        'L' => direction = get_direction_after_rotation(direction, 360 - instruction.1),
        'R' => direction = get_direction_after_rotation(direction, instruction.1),
        'F' => {
            position = get_position_after_move(position, direction, instruction.1 as i32);
        }
        _ => return,
    });

    return position.0.abs() + position.1.abs();
}

fn get_waypoint_position_after_rotation(
    waypoint_position: (i32, i32),
    rotation: usize,
) -> (i32, i32) {
    let mut new_waypoint_position = waypoint_position.clone();
    let number_of_rotations = rotation / 90;

    (0..number_of_rotations).for_each(|i| {
        new_waypoint_position = (new_waypoint_position.1, 0 - new_waypoint_position.0)
    });
    return new_waypoint_position;
}

#[aoc(day12, part2)]
fn solve_part2(input: &[(char, usize)]) -> i32 {
    println!("{:?}", input);

    let mut position = (0, 0);
    let mut waypoint_position = (10, 1);

    input.iter().for_each(|instruction| {
        match instruction.0 {
            'N' | 'E' | 'S' | 'W' => {
                waypoint_position =
                    get_position_after_move(waypoint_position, instruction.0, instruction.1 as i32);
            }
            'L' => {
                waypoint_position =
                    get_waypoint_position_after_rotation(waypoint_position, 360 - instruction.1)
            }
            'R' => {
                waypoint_position =
                    get_waypoint_position_after_rotation(waypoint_position, instruction.1)
            }
            'F' => {
                position =
                    get_position_after_move_waypoint(waypoint_position, position, instruction.1);
            }
            _ => return,
        }
        println!("{:?} {:?} {:?}", instruction, position, waypoint_position);
    });

    return position.0.abs() + position.1.abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"L270
F10
N3
F7
R90
F11
";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 25;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let real_input = r"F10
N3
F7
R90
F11
";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 286;
        assert_eq!(result, expected);
    }
}
