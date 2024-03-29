use std::collections::HashMap;

use regex::Regex;
#[derive(Debug, Clone)]

enum Operator {
    Plus,
    Multiply,
}
#[derive(Debug, Clone)]
enum Argument {
    Variable(String),
    Number(usize),
}
#[derive(Debug, Clone)]

pub struct MonkeyTest {
    divisible_by: usize,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
}
#[derive(Debug, Clone)]

pub struct Monkey {
    items: Vec<usize>,
    operation: (Operator, Argument),
    test: MonkeyTest,
    id: usize,
    num_throws: usize,
}

struct NextMonkey {
    monkey_id: usize,
    worry_score: usize,
}

impl Monkey {
    fn tick(&mut self, lcm: usize, part: usize) -> Vec<NextMonkey> {
        let items = self.items.clone();
        self.num_throws += items.len();
        self.items = vec![];

        let results = items.iter().map(|item| {
            let new_first_item = self.perform_operation(*item);

            let mut bored_first_item = new_first_item / (if part == 1 { 3 } else { 1 });

            if bored_first_item > lcm {
                bored_first_item = bored_first_item % lcm;
            }

            let next_monkey = self.test(bored_first_item);

            NextMonkey {
                monkey_id: next_monkey,
                worry_score: bored_first_item,
            }
        });

        results.collect()
    }

    fn perform_operation(&self, worry_score: usize) -> usize {
        match self.operation {
            (Operator::Plus, Argument::Variable(_)) => worry_score + worry_score,
            (Operator::Plus, Argument::Number(var)) => worry_score + var,
            (Operator::Multiply, Argument::Variable(_)) => worry_score * worry_score,
            (Operator::Multiply, Argument::Number(var)) => worry_score * var,
        }
    }

    fn test(&self, worry_score: usize) -> usize {
        if worry_score % self.test.divisible_by == 0 {
            return self.test.if_true_throw_to;
        } else {
            return self.test.if_false_throw_to;
        }
    }

    fn receive(&mut self, worry_score: usize) {
        self.items.push(worry_score);
    }
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Monkey> {
    let monkey_regex = Regex::new(
        r"Monkey (?P<id>\d*):\n  Starting items: (?P<items>[\d, ]*)\n  Operation: new = old (?P<operator>[+\*]) (?P<math>[\d]*\w*)\n. Test: divisible by (?P<divisible>\d*)\n    If true: throw to monkey (?P<truemonkey>\d*)\n.   If false: throw to monkey (?P<falsemonkey>\d*)",
    ).unwrap();

    input
        .split("\n\n")
        .map(|monkey_info| {
            let captures = monkey_regex.captures(monkey_info).unwrap();

            let monkey_id: usize = captures.name("id").unwrap().as_str().parse().unwrap();
            let starting_items: Vec<usize> = captures
                .name("items")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|item| item.trim().parse().unwrap())
                .collect();

            let operator: Operator = match captures.name("operator").unwrap().as_str() {
                "*" => Operator::Multiply,
                "+" => Operator::Plus,
                _ => panic!("operator failed"),
            };

            let math_value = captures.name("math").unwrap().as_str();
            let operator_pair = match math_value {
                "old" => Argument::Variable("old".to_string()),
                _ => Argument::Number(math_value.parse().unwrap()),
            };

            let divisible_by: usize = captures
                .name("divisible")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            let true_monkey: usize = captures
                .name("truemonkey")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            let false_monkey: usize = captures
                .name("falsemonkey")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            return Monkey {
                items: starting_items,
                operation: (operator, operator_pair),
                test: MonkeyTest {
                    divisible_by,
                    if_true_throw_to: true_monkey,
                    if_false_throw_to: false_monkey,
                },
                id: monkey_id,
                num_throws: 0,
            };
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Monkey]) -> usize {
    let num_monkeys = input.len();
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    let divisors: Vec<usize> = input
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .collect();

    let lcm: usize = divisors.into_iter().product();

    for monkey in input {
        monkeys.insert(monkey.id, monkey.to_owned());
    }

    for _ in 0..20 {
        for monkey_index in 0..num_monkeys {
            let a_monkey = monkeys.get_mut(&monkey_index).unwrap();
            let monkey_results = a_monkey.tick(lcm, 1);

            monkey_results.iter().for_each(|monkey_result| {
                let matching_monkey = monkeys.get_mut(&monkey_result.monkey_id).unwrap();
                matching_monkey.receive(monkey_result.worry_score);
            });
        }
    }

    let mut monkey_business: Vec<usize> =
        monkeys.values().map(|monkey| monkey.num_throws).collect();

    monkey_business.sort();
    monkey_business.reverse();

    monkey_business[0] * monkey_business[1]
}

/// Uses Lowest Common Multiple to make the numbers smaller.
/// We only test if a number is divisible by 11, 5, 3, 19, 2, 13, 7 and 17.
/// Any number that is divisible by any of those numbers, is also divisible by the LCM.
/// Therefore just store the number % LCM instead of the big number.
/// We don't actually use the result of the number!!!
///

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Monkey]) -> usize {
    let num_monkeys = input.len();
    let divisors: Vec<usize> = input
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .collect();

    let lcm: usize = divisors.into_iter().product();

    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    for monkey in input {
        monkeys.insert(monkey.id, monkey.to_owned());
    }

    for _ in 0..10000 {
        for monkey_index in 0..num_monkeys {
            let a_monkey = monkeys.get_mut(&monkey_index).unwrap();
            let monkey_results = a_monkey.tick(lcm, 2);

            monkey_results.iter().for_each(|monkey_result| {
                let matching_monkey = monkeys.get_mut(&monkey_result.monkey_id).unwrap();
                matching_monkey.receive(monkey_result.worry_score);
            });
        }
    }

    let mut monkey_business: Vec<usize> =
        monkeys.values().map(|monkey| monkey.num_throws).collect();

    monkey_business.sort();
    monkey_business.reverse();

    monkey_business[0] * monkey_business[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUTS: &[&str] = &["Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"];

    #[test]
    fn part1_test_1() {
        const TEST_RESULTS: &[usize] = &[10605];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part1(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }

    #[test]
    fn part2_test_1() {
        const TEST_RESULTS: &[usize] = &[2713310158];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part2(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }
}
