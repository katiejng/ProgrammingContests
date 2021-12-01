use std::collections::HashMap;

#[derive(Debug)]
enum InputRule {
    Mask(String),
    Replace(usize, usize),
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<InputRule> {
    input
        .lines()
        .map(|l| {
            let mut split_line = l.split(" = ");
            if l.contains("mask") {
                return InputRule::Mask(split_line.last().unwrap().parse().unwrap());
            } else {
                let first = split_line.next().unwrap();
                return InputRule::Replace(
                    first
                        .split("[")
                        .last()
                        .unwrap()
                        .split("]")
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap(),
                    split_line.next().unwrap().parse().unwrap(),
                );
            }
        })
        .collect()
}

fn get_number_after_mask(mask: &str, value: &str) -> usize {
    let mask_chars = mask.chars();
    let value_chars = value.chars();

    let result: String = mask_chars
        .zip(value_chars)
        .map(|(mask, value)| match mask {
            'X' => return value,
            '0' => return '0',
            '1' => return '1',
            _ => return '0',
        })
        .collect();

    usize::from_str_radix(&result, 2).unwrap()
}

fn get_numbers_after_mask(mask: &str, value: &str) -> Vec<usize> {
    let mask_chars = mask.chars();
    let value_chars = value.chars();

    let result: String = mask_chars
        .zip(value_chars)
        .map(|(mask, value)| match mask {
            'X' => return 'F',
            '0' => return value,
            '1' => return '1',
            _ => return '0',
        })
        .collect();

    let permutations = result.matches('F').count();
    let test = (0..2usize.pow(permutations as u32))
        .map(|value| {
            let binary_value = format!("{:0permutations$b}", value, permutations = permutations);
            let mut binary_chars = binary_value.chars();

            let new_address: String = result
                .chars()
                .map(|c| match c {
                    '1' | '0' => return c,
                    'F' => return binary_chars.next().unwrap(),
                    _ => return '0',
                })
                .collect();

            usize::from_str_radix(&new_address, 2).unwrap()
        })
        .collect();

    return test;
}

#[aoc(day14, part1)]
fn solve_part1(input: &[InputRule]) -> usize {
    let mut results: HashMap<usize, usize> = HashMap::new();

    let mut mask = "";
    input.iter().for_each(|input_line| match input_line {
        InputRule::Mask(mask_string) => {
            mask = mask_string;
        }
        InputRule::Replace(position, value) => {
            let binary_string = format!("{:0>36b}", value);
            let pointer = results.entry(*position).or_insert(0);
            *pointer = get_number_after_mask(mask, &binary_string);
        }
    });
    return results.values().sum();
}

#[aoc(day14, part2)]
fn solve_part2(input: &[InputRule]) -> usize {
    let mut results: HashMap<usize, usize> = HashMap::new();

    let mut mask = "";
    input.iter().for_each(|input_line| match input_line {
        InputRule::Mask(mask_string) => {
            mask = mask_string;
        }
        InputRule::Replace(position, value) => {
            let binary_string = format!("{:0>36b}", position);
            let pointers_value = get_numbers_after_mask(mask, &binary_string);
            pointers_value.iter().for_each(|pointer_value| {
                let pointer = results.entry(*pointer_value).or_insert(0);
                *pointer = *value;
            });
        }
    });
    return results.values().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 165;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let real_input = r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 208;
        assert_eq!(result, expected);
    }
}
