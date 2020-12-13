#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn get_neighbours(input: &[Vec<char>], row: isize, col: isize) -> Vec<char> {
    let num_rows = input.len() as isize;
    let num_cols = input[0].len() as isize;

    let mut result: Vec<char> = vec![];
    for r in row - 1..row + 2 {
        for c in col - 1..col + 2 {
            if r == row && c == col {
                continue;
            }

            if r >= 0 && r < num_rows && c >= 0 && c < num_cols {
                let value = input[r as usize][c as usize];
                result.push(value);
            }
        }
    }

    return result;
}

fn get_visible_seats(input: &[Vec<char>], row: isize, col: isize) -> Vec<char> {
    let num_rows = input.len() as isize;
    let num_cols = input[0].len() as isize;

    let mut result: Vec<char> = vec![];

    (-1..2).for_each(|r| {
        (-1..2).for_each(|c| {
            if r == 0 && c == 0 {
                return;
            }
            let mut multiplier = 1;

            loop {
                let temp_row = row + r * multiplier;
                let temp_col = col + c * multiplier;

                if temp_row >= 0 && temp_row < num_rows && temp_col >= 0 && temp_col < num_cols {
                    let seat_state: char = input[temp_row as usize][temp_col as usize];
                    if seat_state == 'L' || seat_state == '#' {
                        result.push(seat_state);
                        break;
                    }
                } else {
                    break;
                }
                multiplier += 1;
            }
        });
    });

    return result;
}

fn get_sum_in_row(input: &[char]) -> usize {
    input.iter().filter(|character| *character == &'#').count()
}
fn get_sum(input: &[Vec<char>]) -> usize {
    input.iter().map(|row| get_sum_in_row(&row)).sum()
}

#[aoc(day11, part1)]
fn solve_part1(input: &[Vec<char>]) -> usize {
    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut current: Vec<Vec<char>> = input.clone().to_vec();

    loop {
        let mut next_step = current.clone().to_vec();

        (0..num_rows).for_each(|row| {
            (0..num_cols).for_each(|col| {
                let current_seat_state = current[row][col];
                let neighbour_count =
                    get_sum_in_row(&get_neighbours(&current, row as isize, col as isize));

                if current_seat_state == 'L' && neighbour_count == 0 {
                    next_step[row][col] = '#';
                } else if current_seat_state == '#' && neighbour_count >= 4 {
                    next_step[row][col] = 'L';
                }
            });
        });
        let sum_seated: usize = get_sum(&current);
        let new_sum_seated = get_sum(&next_step);
        if sum_seated == new_sum_seated {
            return sum_seated;
        }
        current = next_step;
    }
}

#[aoc(day11, part2)]
fn solve_part2(input: &[Vec<char>]) -> usize {
    let num_rows = input.len();
    let num_cols = input[0].len();
    let mut current: Vec<Vec<char>> = input.clone().to_vec();
    loop {
        let mut next_step = current.clone().to_vec();

        (0..num_rows).for_each(|row| {
            (0..num_cols).for_each(|col| {
                let current_seat_state = current[row][col];
                let neighbour_count =
                    get_sum_in_row(&get_visible_seats(&current, row as isize, col as isize));

                if current_seat_state == 'L' && neighbour_count == 0 {
                    next_step[row][col] = '#';
                } else if current_seat_state == '#' && neighbour_count >= 5 {
                    next_step[row][col] = 'L';
                }
            });
        });
        let sum_seated: usize = get_sum(&current);
        if sum_seated == get_sum(&next_step) {
            return sum_seated;
        }
        current = next_step;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 37;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let real_input = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 26;
        assert_eq!(result, expected);
    }
}
