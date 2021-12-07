use std::collections::HashMap;

pub struct Board {
    numbers: Vec<Vec<BoardItem>>,
}
pub struct BoardItem {
    number: usize,
    order: usize,
}
#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Board> {
    let mut split_input = input.lines();
    let order = split_input.next().unwrap();

    let mut positions: HashMap<usize, usize> = HashMap::new();
    order.split(",").enumerate().for_each(|(index, item)| {
        positions.insert(item.trim().parse().unwrap(), index);
    });

    let mut split_double = input.split("\n\n");
    split_double.next(); // ignore first
    let parsed: Vec<Vec<Vec<BoardItem>>> = split_double
        .into_iter()
        .map(|item| {
            let temp_board: Vec<Vec<BoardItem>> = item
                .lines()
                .map(|l| {
                    l.split_whitespace()
                        .map(|i| {
                            let num: usize = i.trim().parse().unwrap();
                            BoardItem {
                                number: num,
                                order: positions.get(&num).unwrap_or(&1000).clone(),
                            }
                        })
                        .collect()
                })
                .collect();
            temp_board
        })
        .collect();

    let boards = parsed.into_iter().map(|i| Board { numbers: i }).collect();

    boards
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Board]) -> usize {
    let winning_turns: Vec<usize> = (0..input.len())
        .map(|b| get_winning_turn(input, b))
        .collect();

    let (winning_board_index, winning_turn) = winning_turns
        .iter()
        .enumerate()
        .min_by_key(|item| item.1)
        .unwrap();

    let winning_board = &input[winning_board_index].numbers;
    let numbers: Vec<usize> = winning_board
        .iter()
        .flat_map(|row| {
            let row_total: Vec<usize> = row
                .iter()
                .filter(|&item| item.order > *winning_turn)
                .map(|item| item.number)
                .collect();

            row_total
        })
        .collect();

    let total_sum: usize = numbers.iter().sum();

    let winning_spot: Vec<&BoardItem> = winning_board
        .iter()
        .flat_map(|row| row.iter().filter(|i| i.order == *winning_turn))
        .collect();
    return total_sum * winning_spot[0].number;
}

fn get_winning_turn(input: &[Board], b: usize) -> usize {
    let mut min_winner_total = usize::MAX;
    let current_board = &input[b].numbers;

    (0..5).for_each(|r| {
        let max = (0..5).map(|c| current_board[r][c].order).max().unwrap();
        if max < min_winner_total {
            min_winner_total = max;
        }
    });

    (0..5).for_each(|c| {
        let max = (0..5).map(|r| current_board[r][c].order).max().unwrap();
        if max < min_winner_total {
            min_winner_total = max;
        }
    });

    min_winner_total
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Board]) -> usize {
    let winning_turns: Vec<usize> = (0..input.len())
        .map(|b| get_winning_turn(input, b))
        .collect();

    let (losing_board_index, losing_turn) = winning_turns
        .iter()
        .enumerate()
        .max_by_key(|item| item.1)
        .unwrap();

    let winning_board = &input[losing_board_index].numbers;
    let numbers: Vec<usize> = winning_board
        .iter()
        .flat_map(|row| {
            let row_total: Vec<usize> = row
                .iter()
                .filter(|&item| item.order > *losing_turn)
                .map(|item| item.number)
                .collect();

            row_total
        })
        .collect();

    let total_sum: usize = numbers.iter().sum();

    let winning_spot: Vec<&BoardItem> = winning_board
        .iter()
        .flat_map(|row| row.iter().filter(|i| i.order == *losing_turn))
        .collect();
    return total_sum * winning_spot[0].number;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let parsed_input = input_generator(input);
        let result = solve_part1(&parsed_input);
        let expected = 4512;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let parsed_input = input_generator(input);
        let result = solve_part2(&parsed_input);
        let expected = 1924;
        assert_eq!(result, expected);
    }
}
