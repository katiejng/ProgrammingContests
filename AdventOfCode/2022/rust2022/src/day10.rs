#[derive(Debug)]
pub enum CommandType {
    AddX(isize),
    Noop,
}

pub struct Computer {
    current_cycle: isize,
    x_reg: isize,
    results: isize,
}

impl Computer {
    fn new() -> Self {
        Computer {
            current_cycle: 1,
            x_reg: 1,
            results: 0,
        }
    }

    fn tick(&mut self) {
        if self.is_special_cycle() {
            self.results += self.get_signal_strength();
        }
        self.print_char();
        self.current_cycle += 1;
    }

    fn print_char(&mut self) {
        let cycle = self.current_cycle;
        let position = (cycle - 1) % 40;
        if position == 0 {
            print!("\n");
        }
        let x_value = self.x_reg;
        let is_covered = (x_value - position).abs() <= 1;
        if is_covered {
            print!("#");
        } else {
            print!(" ");
        }
    }

    fn run_command(&mut self, command: &CommandType) {
        match command {
            CommandType::AddX(value) => {
                self.tick();
                self.tick();
                self.x_reg += value;
            }
            CommandType::Noop => self.tick(),
        }
    }

    fn is_special_cycle(&self) -> bool {
        (self.current_cycle - 20) % 40 == 0
    }

    fn get_signal_strength(&self) -> isize {
        self.current_cycle * self.x_reg
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<CommandType> {
    input
        .lines()
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();
            match split[0] {
                "noop" => CommandType::Noop,
                "addx" => CommandType::AddX(split[1].parse().unwrap()),
                _ => panic!("not a valid command"),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[CommandType]) -> isize {
    let mut computer = Computer::new();
    input.iter().for_each(|command| {
        computer.run_command(&command);
    });
    computer.results
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[CommandType]) -> isize {
    let mut computer = Computer::new();
    input.iter().for_each(|command| {
        computer.run_command(&command);
    });
    computer.results
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUTS: &[&str] = &["addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"];

    #[test]
    fn part1_test_1() {
        const TEST_RESULTS: &[isize] = &[13140];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part1(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }

    #[test]
    fn part2_test_1() {
        const TEST_RESULTS: &[isize] = &[13140];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part2(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }
}
