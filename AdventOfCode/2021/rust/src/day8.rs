use std::collections::HashSet;
struct InputRule {
    command: String,
    value: isize,
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<InputRule> {
    input
        .lines()
        .map(|l| {
            let mut split_line = l.split(" ");
            InputRule {
                command: String::from(split_line.next().unwrap()),
                value: split_line.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &[InputRule]) -> isize {
    input.iter().for_each(|line| {
        println!("{}|{}", line.command, line.value);
    });

    let mut acc = 0;

    let mut step = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    loop {
        let current_step = input.get(step).unwrap();
        if visited.contains(&step) {
            break;
        }
        visited.insert(step);

        match current_step.command.as_str() {
            "acc" => {
                acc += current_step.value;
                step += 1;
            }
            "jmp" => {
                let temp_step: isize = step as isize + current_step.value;
                step = temp_step as usize;
            }
            _ => {
                step += 1;
            }
        }
    }
    return acc;
}

fn helper(input: Vec<InputRule>) -> (bool, isize) {
    input.iter().for_each(|line| {
        println!("{}|{}", line.command, line.value);
    });

    let mut acc = 0;

    let mut step = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    loop {
        if step >= input.len() {
            break;
        }
        let current_step = input.get(step).unwrap();
        if visited.contains(&step) {
            return (false, acc);
        }
        visited.insert(step);

        match current_step.command.as_str() {
            "acc" => {
                acc += current_step.value;
                step += 1;
            }
            "jmp" => {
                let temp_step: isize = step as isize + current_step.value;
                step = temp_step as usize;
            }
            _ => {
                step += 1;
            }
        }
    }
    return (true, acc);
}

#[aoc(day8, part2)]
fn solve_part2(input: &[InputRule]) -> isize {
    input.iter().for_each(|line| {
        println!("{}|{}", line.command, line.value);
    });

    for num in 0..input.len() - 1 {
        let temp: Vec<InputRule> = input
            .iter()
            .enumerate()
            .map(|(index, line)| {
                if index == num {
                    if line.command == String::from("nop") {
                        return InputRule {
                            command: String::from("jmp"),
                            value: line.value,
                        };
                    } else if line.command == String::from("jmp") {
                        return InputRule {
                            command: String::from("nop"),
                            value: line.value,
                        };
                    }
                }
                return InputRule {
                    command: String::from(line.command.to_string()),
                    value: line.value,
                };
            })
            .collect();

        let (result, acc) = helper(temp);
        if result {
            return acc;
        }
    }

    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let real_input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 8;
        assert_eq!(result, expected);
    }
}
