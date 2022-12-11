use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct Movement {
    direction: Direction,
    amount: usize,
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Movement> {
    input
        .lines()
        .map(|l| {
            let result: Vec<&str> = l.split_whitespace().collect();
            Movement {
                direction: match result[0] {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("not a direction"),
                },
                amount: result[1].parse().expect("amount not a num"),
            }
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Movement]) -> usize {
    let mut head_x: isize = 0;
    let mut head_y: isize = 0;
    let mut tail_x: isize = 0;
    let mut tail_y: isize = 0;

    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    visited.insert((head_x, head_y));

    for line in input {
        (0..line.amount).for_each(|_| {
            (head_x, head_y) = match line.direction {
                Direction::Up => (head_x, head_y + 1),
                Direction::Down => (head_x, head_y - 1),
                Direction::Left => (head_x - 1, head_y),
                Direction::Right => (head_x + 1, head_y),
            };

            (tail_x, tail_y) = get_next_tail_position((head_x, head_y), (tail_x, tail_y));

            visited.insert((tail_x, tail_y));
        });
    }

    visited.len()
}

fn get_next_tail_position(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    let (head_x, head_y) = head;
    let (tail_x, tail_y) = tail;
    let x_abs = (head_x - tail_x).abs();
    let y_abs = (head_y - tail_y).abs();

    // touching
    if x_abs <= 1 && y_abs <= 1 {
        return (tail_x, tail_y);
    }

    let x_movement = if head_x - tail_x > 0 { 1 } else { -1 };
    let y_movement = if head_y - tail_y > 0 { 1 } else { -1 };

    // if 2 spots away, bring 1 closer
    if x_abs == 0 && y_abs >= 2 {
        return (tail_x, tail_y + y_movement);
    }
    if y_abs == 0 && x_abs >= 2 {
        return (tail_x + x_movement, tail_y);
    }

    // diagonals
    return (tail_x + x_movement, tail_y + y_movement);
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Movement]) -> usize {
    let num_rope_links = 10;
    let mut points: Vec<(isize, isize)> = vec![];

    for _ in 0..num_rope_links {
        points.push((0, 0));
    }

    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    visited.insert((0, 0));

    for line in input {
        (0..line.amount).for_each(|_| {
            let (head_x, head_y) = points[0];
            points[0] = match line.direction {
                Direction::Up => (head_x, head_y + 1),
                Direction::Down => (head_x, head_y - 1),
                Direction::Left => (head_x - 1, head_y),
                Direction::Right => (head_x + 1, head_y),
            };

            (1..num_rope_links).for_each(|index| {
                points[index] = get_next_tail_position(points[index - 1], points[index]);
            });

            visited.insert(points.last().unwrap().clone());
        });
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUTS: &[&str] = &[
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
    ];

    #[test]
    fn part1_test_1() {
        const TEST_RESULTS: &[usize] = &[13, 88];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part1(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }

    #[test]
    fn part2_test_1() {
        const TEST_RESULTS: &[usize] = &[1, 36];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part2(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }
}
