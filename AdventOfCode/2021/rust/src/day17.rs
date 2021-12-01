#[aoc_generator(day17)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn get_neighbours(input: &Vec<Vec<Vec<usize>>>, z_dim: i32, row: i32, col: i32) -> Vec<usize> {
    let num_z_dim = input.len() as i32;
    let num_rows = input[0].len() as i32;
    let num_cols = input[0][0].len() as i32;

    let mut result: Vec<usize> = vec![];
    for z in z_dim - 1..z_dim + 2 {
        for r in row - 1..row + 2 {
            for c in col - 1..col + 2 {
                if r == row && c == col && z == z_dim {
                    continue;
                }

                if z >= 0 && z < num_z_dim && r >= 0 && r < num_rows && c >= 0 && c < num_cols {
                    let value: usize = input[z as usize][r as usize][c as usize];

                    result.push(value);
                }
            }
        }
    }

    return result;
}

fn get_neighbours_four(
    input: &Vec<Vec<Vec<Vec<usize>>>>,
    w_dim: i32,
    z_dim: i32,
    row: i32,
    col: i32,
) -> Vec<usize> {
    let num_w_dim = input.len() as i32;
    let num_z_dim = input[0].len() as i32;
    let num_rows = input[0][0].len() as i32;
    let num_cols = input[0][0][0].len() as i32;

    let mut result: Vec<usize> = vec![];
    for w in w_dim - 1..w_dim + 2 {
        for z in z_dim - 1..z_dim + 2 {
            for r in row - 1..row + 2 {
                for c in col - 1..col + 2 {
                    if w == w_dim && r == row && c == col && z == z_dim {
                        continue;
                    }

                    if w >= 0
                        && w < num_w_dim
                        && z >= 0
                        && z < num_z_dim
                        && r >= 0
                        && r < num_rows
                        && c >= 0
                        && c < num_cols
                    {
                        let value: usize = input[w as usize][z as usize][r as usize][c as usize];

                        result.push(value);
                    }
                }
            }
        }
    }

    return result;
}

fn get_sum_in_row(input: &[usize]) -> usize {
    input.iter().sum()
}
fn get_sum(input: &Vec<Vec<usize>>) -> usize {
    input.iter().map(|row| get_sum_in_row(&row)).sum()
}

fn get_sum_z_dim(input: &Vec<Vec<Vec<usize>>>) -> usize {
    input.iter().map(|row| get_sum(row)).sum()
}

fn get_sum_w_dim(input: &Vec<Vec<Vec<Vec<usize>>>>) -> usize {
    input.iter().map(|row| get_sum_z_dim(row)).sum()
}

fn _print_nicely(current: Vec<Vec<Vec<usize>>>) {
    println!("STARTING");
    current.iter().enumerate().for_each(|(i, item)| {
        item.iter().for_each(|item1| {
            println!("{:?}", item1);
        });
        println!("layer {}", i);
    });
}

#[aoc(day17, part1)]
fn solve_part1(input: &[Vec<char>]) -> usize {
    let rounds = 7;
    let init_rows = input.len();
    let init_cols = input[0].len();
    let num_z_dim = 1 + 2 * rounds;
    let num_rows = input.len() + 2 * rounds;
    let num_cols = input[0].len() + 2 * rounds;

    let mut current: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; num_cols]; num_rows]; num_z_dim];

    (rounds..init_rows + rounds).for_each(|row| {
        (rounds..init_cols + rounds).for_each(|col| {
            current[rounds][row][col] = if input[row - rounds][col - rounds] == '#' {
                1
            } else {
                0
            };
        })
    });
    // print_nicely(current.clone());

    let mut count = 0;
    loop {
        count += 1;
        if count == rounds {
            break;
        }
        let mut next_step = current.clone().to_vec();

        (0..num_z_dim).for_each(|z_dim| {
            (0..num_rows).for_each(|row| {
                (0..num_cols).for_each(|col| {
                    let current_seat_state = current[z_dim][row][col];
                    let neighbours: Vec<usize> =
                        get_neighbours(&current, z_dim as i32, row as i32, col as i32);

                    let neighbour_count: usize = neighbours.iter().sum();

                    if current_seat_state == 1 && !(neighbour_count == 2 || neighbour_count == 3) {
                        next_step[z_dim][row][col] = 0;
                    } else if current_seat_state == 0 && neighbour_count == 3 {
                        next_step[z_dim][row][col] = 1;
                    }
                });
            });
        });
        current = next_step;
        // print_nicely(current.clone());
    }

    return get_sum_z_dim(&current.clone());
}

#[aoc(day17, part2)]
fn solve_part2(input: &[Vec<char>]) -> usize {
    let rounds = 7;
    let init_rows = input.len();
    let init_cols = input[0].len();
    let num_z_dim = 1 + 2 * rounds;
    let num_w_dim = 1 + 2 * rounds;
    let num_rows = input.len() + 2 * rounds;
    let num_cols = input[0].len() + 2 * rounds;

    let mut current: Vec<Vec<Vec<Vec<usize>>>> =
        vec![vec![vec![vec![0; num_cols]; num_rows]; num_z_dim]; num_w_dim];

    (rounds..init_rows + rounds).for_each(|row| {
        (rounds..init_cols + rounds).for_each(|col| {
            current[rounds][rounds][row][col] = if input[row - rounds][col - rounds] == '#' {
                1
            } else {
                0
            };
        })
    });
    // print_nicely(current.clone());

    let mut count = 0;
    loop {
        count += 1;
        if count == rounds {
            break;
        }
        let mut next_step = current.clone().to_vec();

        (0..num_w_dim).for_each(|w_dim| {
            (0..num_z_dim).for_each(|z_dim| {
                (0..num_rows).for_each(|row| {
                    (0..num_cols).for_each(|col| {
                        let current_seat_state = current[w_dim][z_dim][row][col];
                        let neighbours: Vec<usize> = get_neighbours_four(
                            &current,
                            w_dim as i32,
                            z_dim as i32,
                            row as i32,
                            col as i32,
                        );

                        let neighbour_count: usize = neighbours.iter().sum();

                        if current_seat_state == 1
                            && !(neighbour_count == 2 || neighbour_count == 3)
                        {
                            next_step[w_dim][z_dim][row][col] = 0;
                        } else if current_seat_state == 0 && neighbour_count == 3 {
                            next_step[w_dim][z_dim][row][col] = 1;
                        }
                    });
                });
            });
        });
        current = next_step;
        // print_nicely(current.clone());
    }

    return get_sum_w_dim(&current.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r".#.
..#
###";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 112;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let real_input = r".#.
..#
###";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 848;
        assert_eq!(result, expected);
    }
}
