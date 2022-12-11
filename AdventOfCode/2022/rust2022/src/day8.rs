#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_row_col_at(r: usize, c: usize, len: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (len - c - 1, r),
        Direction::Down => (c, r),
        Direction::Left => (r, len - c - 1),
        Direction::Right => (r, c),
    }
}

fn check_direction(
    mut results: Vec<Vec<usize>>,
    input: &[Vec<u32>],
    direction: Direction,
) -> Vec<Vec<usize>> {
    let len = input.len();

    for r in 0..len {
        let mut previous = 0;
        for c in 0..len {
            let (row, col) = get_row_col_at(r, c, len, direction);

            if c == 0 {
                results[row][col] = 1;
            } else if input[row][col] > previous {
                results[row][col] = 1;
            }
            if input[row][col] > previous {
                previous = input[row][col]
            }
        }
    }
    results
}

fn check_visible(mut results: Vec<Vec<usize>>, input: &[Vec<u32>]) -> Vec<Vec<usize>> {
    results = check_direction(results, input, Direction::Right);
    results = check_direction(results, input, Direction::Left);
    results = check_direction(results, input, Direction::Up);
    results = check_direction(results, input, Direction::Down);

    results
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> usize {
    let len: usize = input.len() as usize;

    let mut results: Vec<Vec<usize>> = vec![];
    for _ in 0..len {
        results.push(vec![0; len])
    }

    results = check_visible(results, input);

    results
        .into_iter()
        .map(|row| row.into_iter().sum::<usize>())
        .sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> usize {
    let len: usize = input.len() as usize;

    let mut results: Vec<Vec<usize>> = vec![];
    for _ in 0..len {
        results.push(vec![0; len])
    }

    for r in 0..len {
        for c in 0..len {
            results[r][c] = get_spot_visibility(r, c, input);
        }
    }

    results.iter().flat_map(|row| row).max().unwrap().clone()
}

fn get_visibility_in_direction(
    r: usize,
    c: usize,
    input: &[Vec<u32>],
    direction: Direction,
) -> usize {
    let len = input.len() as isize;
    let current_height = input[r][c];
    let mut total = 0;
    let mut r_pointer: isize = r as isize;
    let mut c_pointer: isize = c as isize;

    loop {
        (r_pointer, c_pointer) = match direction {
            Direction::Up => (r_pointer - 1, c_pointer),
            Direction::Down => (r_pointer + 1, c_pointer),
            Direction::Left => (r_pointer, c_pointer - 1),
            Direction::Right => (r_pointer, c_pointer + 1),
        };

        if r_pointer >= len || r_pointer < 0 || c_pointer >= len || c_pointer < 0 {
            break;
        }

        let looking_tree = input[r_pointer as usize][c_pointer as usize];
        if looking_tree < current_height {
            total += 1;
        } else {
            total += 1;
            break;
        }
    }

    total
}

fn get_spot_visibility(r: usize, c: usize, input: &[Vec<u32>]) -> usize {
    let right = get_visibility_in_direction(r, c, input, Direction::Right);
    let up = get_visibility_in_direction(r, c, input, Direction::Up);
    let down = get_visibility_in_direction(r, c, input, Direction::Down);
    let left = get_visibility_in_direction(r, c, input, Direction::Left);
    right * up * down * left
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUTS: &[&str] = &["30373
25512
65332
33549
35390"];

    #[test]
    fn part1_test_1() {
        const TEST_RESULTS: &[usize] = &[21];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part1(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }

    #[test]
    fn part2_test_1() {
        const TEST_RESULTS: &[usize] = &[8];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part2(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }
}
