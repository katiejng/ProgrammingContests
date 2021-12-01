#[aoc_generator(day22)]
fn input_generator(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut split_input = input.lines();
    let mut player1: Vec<usize> = vec![];
    let mut player2: Vec<usize> = vec![];

    let mut mode = 0;
    loop {
        let line = split_input.next();
        match line {
            Some(value) => match value.trim() {
                "Player 1:" => {
                    mode = 0;
                }
                "Player 2:" => {
                    mode = 1;
                }
                "" => {
                    continue;
                }
                _ => {
                    let num: usize = value.trim().parse().unwrap();
                    if mode == 0 {
                        player1.insert(0, num);
                    } else {
                        player2.insert(0, num);
                    }
                }
            },
            None => {
                break;
            }
        }
    }

    return (player1, player2);
}

#[aoc(day22, part1)]
fn solve_part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut input_copy = input.clone();

    while input_copy.0.len() != 0 && input_copy.1.len() != 0 {
        let a = input_copy.0.pop().unwrap();
        let b = input_copy.1.pop().unwrap();

        if a > b {
            input_copy.0.insert(0, a);
            input_copy.0.insert(0, b);
        } else {
            input_copy.1.insert(0, b);
            input_copy.1.insert(0, a);
        }
    }

    let result = if input_copy.0.len() == 0 {
        input_copy.1
    } else {
        input_copy.0
    };
    return result
        .iter()
        .enumerate()
        .map(|(index, value)| (index + 1) * value)
        .sum();
}

#[aoc(day22, part2)]
fn solve_part2(_input: &(Vec<usize>, Vec<usize>)) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 306;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let real_input = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 1;
        assert_eq!(result, expected);
    }
}
