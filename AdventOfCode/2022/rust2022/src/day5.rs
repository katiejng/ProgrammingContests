use regex::Regex;

#[derive(Debug)]
pub struct Moves {
    items_to_move: usize,
    start_stack: usize,
    end_stack: usize,
}

#[derive(Debug)]
pub struct Input {
    containers: Vec<Vec<char>>,
    moves: Vec<Moves>,
}

#[aoc_generator(day5)]
fn input_generator(_input: &str) -> Input {
    let (container_section, move_section) = _input.split_once("\n\n").unwrap();

    let last_line = container_section.lines().last().unwrap();
    let last_line_parsed: Vec<usize> = last_line
        .split_whitespace()
        .map(|numbers| numbers.trim().parse().unwrap())
        .collect();

    let number_of_stacks: usize = *last_line_parsed.last().unwrap();
    let largest_stack: usize = container_section.lines().collect::<Vec<&str>>().len() - 1;

    let split_containers: Vec<Vec<char>> = container_section
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let containers: Vec<Vec<char>> = (0..number_of_stacks)
        .map(|stack_num| {
            (0..largest_stack)
                .map(|stack_height| {
                    let item_at_position = split_containers[stack_height][stack_num * 4 + 1];

                    return item_at_position;
                })
                .filter(|&character| character != ' ')
                .rev()
                .collect()
        })
        .collect();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let moves: Vec<Moves> = re
        .captures_iter(move_section)
        .map(|capture| {
            return Moves {
                items_to_move: capture[1].trim().parse().unwrap(),
                start_stack: capture[2].trim().parse().unwrap(),
                end_stack: capture[3].trim().parse().unwrap(),
            };
        })
        .collect();

    Input { moves, containers }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> String {
    let mut containers: Vec<Vec<char>> = input.containers.to_owned();

    input.moves.iter().for_each(|a_move| {
        (0..a_move.items_to_move).for_each(|_i| {
            let moved_item = containers[a_move.start_stack - 1].pop().unwrap();
            containers[a_move.end_stack - 1].push(moved_item);
        })
    });

    let results: Vec<String> = containers
        .into_iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect();

    let string: String = results.join("");

    return string;
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> String {
    let mut containers: Vec<Vec<char>> = input.containers.to_owned();

    input.moves.iter().for_each(|a_move| {
        let items_to_move: Vec<char> = (0..a_move.items_to_move)
            .map(|_i| containers[a_move.start_stack - 1].pop().unwrap())
            .collect();

        items_to_move.into_iter().rev().for_each(|item_to_move| {
            containers[a_move.end_stack - 1].push(item_to_move);
        })
    });

    let results: Vec<String> = containers
        .into_iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect();

    let string: String = results.join("");

    return string;
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_test_1() {
        let parsed_input = input_generator(TEST_INPUT);
        // result [['N', 'Z'], ['D', 'C', 'M'], ['P']]
        // moves [Moves { items_to_move: 1, start_stack: 2, end_stack: 1 }, Moves { items_to_move: 3, start_stack: 1, end_stack: 3 }, Moves { items_to_move: 2, start_stack: 2, end_stack: 1 }, Moves { items_to_move: 1, start_stack: 1, end_stack: 2 }]
        let result = solve_part1(&parsed_input);
        let expected = "CMZ";
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let parsed_input = input_generator(TEST_INPUT);
        let result = solve_part2(&parsed_input);
        let expected = "MCD";
        assert_eq!(result, expected);
    }
}
