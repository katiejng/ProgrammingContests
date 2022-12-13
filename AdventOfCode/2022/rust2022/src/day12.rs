use std::{collections::HashSet, iter::FromIterator, vec};

pub struct Input {
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<usize>>,
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Input {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let map = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            let char_line: Vec<usize> = line
                .chars()
                .enumerate()
                .map(|(c, char)| {
                    if char == 'S' {
                        start = (r, c);
                        return 0;
                    }
                    if char == 'E' {
                        end = (r, c);
                        return 25;
                    }

                    char as usize - 'a' as usize
                })
                .collect();
            char_line
        })
        .collect();

    Input { start, end, map }
}

fn get_orthogonal_positions(
    current_position: (usize, usize),
    max_position: (usize, usize),
) -> Vec<(usize, usize)> {
    let (row, column) = current_position;
    let (max_row, max_col) = max_position;
    let mut results = vec![];

    if row > 0 {
        results.push((row - 1, column));
    }
    if column > 0 {
        results.push((row, column - 1));
    }
    if row < max_row {
        results.push((row + 1, column));
    }
    if column < max_col {
        results.push((row, column + 1));
    }

    return results;
}

fn get_path(
    input: &Input,
    distances: &mut Vec<Vec<isize>>,
    current_positions: HashSet<(usize, usize)>,
    max_position: (usize, usize),
    part_1: bool,
) {
    let potential_paths: Vec<(usize, usize)> = current_positions
        .into_iter()
        .flat_map(|position| {
            let current_height = input.map[position.0][position.1];
            let current_distance = distances[position.0][position.1];

            let potential_paths = get_orthogonal_positions(position, max_position);

            let climbable_paths = potential_paths.into_iter().filter(|point| {
                let point_height = input.map[point.0][point.1];

                if part_1 {
                    return point_height <= current_height + 1;
                } else {
                    return current_height <= point_height || current_height == point_height + 1;
                }
            });

            let unvisited_paths: Vec<(usize, usize)> = climbable_paths
                .into_iter()
                .filter(|point| distances[point.0][point.1] == -1)
                .collect();

            unvisited_paths.iter().for_each(|point| {
                distances[point.0][point.1] = current_distance + 1;
            });

            unvisited_paths
        })
        .collect();

    if potential_paths.len() == 0 {
        return;
    }

    let next_positions: HashSet<(usize, usize)> =
        HashSet::from_iter(potential_paths.into_iter().clone());

    get_path(input, distances, next_positions, max_position, part_1);
}

#[aoc(day12, part1)]
/**
   [0, 0, 1, 16, 15, 14, 13, 12]
   [0, 1, 2, 17, 24, 23, 23, 11]
   [0, 2, 2, 18, 25, 25, 23, 10]
   [0, 2, 2, 19, 20, 21, 22,  9]
   [0, 1, 3,  4,  5,  6,  7,  8]
   (0, 0) (2, 5)
*/
pub fn solve_part1(input: &Input) -> usize {
    let rows = input.map.len();
    let cols = input.map[0].len();

    let mut distances: Vec<Vec<isize>> = vec![];
    for _ in 0..rows {
        distances.push(vec![-1; cols])
    }

    distances[input.start.0][input.start.1] = 0;

    let mut current_positions = HashSet::new();
    current_positions.insert(input.start);
    let max_position = (rows - 1, cols - 1);
    get_path(input, &mut distances, current_positions, max_position, true);

    return distances[input.end.0][input.end.1] as usize;
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Input) -> isize {
    let rows = input.map.len();
    let cols = input.map[0].len();

    let mut distances: Vec<Vec<isize>> = vec![];
    for _ in 0..rows {
        distances.push(vec![-1; cols])
    }

    distances[input.end.0][input.end.1] = 0;

    let mut current_positions = HashSet::new();
    current_positions.insert(input.end);
    let max_position = (rows - 1, cols - 1);
    get_path(
        input,
        &mut distances,
        current_positions,
        max_position,
        false,
    );

    let mut least_steps = isize::MAX;

    for r in 0..rows {
        for c in 0..cols {
            if input.map[r][c] == 0 {
                let distance = distances[r][c];
                if distance == -1 {
                    break;
                }
                if distance < least_steps {
                    least_steps = distance;
                }
            }
        }
    }

    least_steps
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUTS: &[&str] = &["Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"];

    #[test]
    fn part1_test_1() {
        const TEST_RESULTS: &[usize] = &[31];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part1(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }

    #[test]
    fn part2_test_1() {
        const TEST_RESULTS: &[isize] = &[29];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part2(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }
}
