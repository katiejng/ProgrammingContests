use std::vec;

// #[derive(Debug)]
// enum InputRule {
//     Mask(String),
//     Replace(usize, usize),
// }
#[derive(Debug, Clone)]
struct RangeRule {
    start: usize,
    end: usize,
}
#[derive(Debug, Clone)]
struct FieldRule {
    label: String,
    ranges: Vec<RangeRule>,
}

#[derive(Debug, Clone)]
struct InputRules {
    field_rules: Vec<FieldRule>,
    tickets: Vec<Vec<usize>>,
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> InputRules {
    let mut split_input = input.split("your ticket:\n");
    let field_rules: Vec<FieldRule> = split_input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut split_field = l.split(": ");
            let label = split_field.next().unwrap();
            let ranges = split_field
                .next()
                .unwrap()
                .split(" or ")
                .map(|range| {
                    let values: Vec<usize> =
                        range.split("-").map(|num| num.parse().unwrap()).collect();
                    return RangeRule {
                        start: values[0],
                        end: values[1],
                    };
                })
                .collect();

            return FieldRule {
                label: String::from(label),
                ranges: ranges,
            };
        })
        .collect();

    let mut tickets = split_input.next().unwrap().split("nearby tickets:\n");
    let my_ticket: Vec<usize> = tickets
        .next()
        .unwrap()
        .split(",")
        .map(|value| value.trim().parse().unwrap())
        .collect();
    let mut nearby_tickets: Vec<Vec<usize>> = tickets
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(",").map(|value| value.parse().unwrap()).collect())
        .collect();

    nearby_tickets.push(my_ticket);
    InputRules {
        field_rules: field_rules,
        tickets: nearby_tickets,
    }
}

fn test_num(input: &InputRules, num: &usize) -> bool {
    input.field_rules.iter().any(|field_rule| {
        field_rule
            .ranges
            .iter()
            .any(|field_rule_range| *num >= field_rule_range.start && *num <= field_rule_range.end)
    })
}

fn test_num_and_field_rule(field_rule: &FieldRule, num: &usize) -> bool {
    field_rule
        .ranges
        .iter()
        .any(|field_rule_range| *num >= field_rule_range.start && *num <= field_rule_range.end)
}

#[aoc(day16, part1)]
fn solve_part1(input: &InputRules) -> usize {
    let mut invalid_numbers: Vec<usize> = vec![];

    input.tickets.iter().for_each(|ticket| {
        ticket.iter().for_each(|num| {
            if !test_num(input, num) {
                invalid_numbers.push(*num);
            }
        })
    });

    return invalid_numbers.iter().sum();
}

fn match_field(tickets: &Vec<Vec<usize>>, field_rule: &FieldRule) -> Vec<usize> {
    let remaining_fields = tickets[0].len();

    let matching_index = (0..remaining_fields)
        .filter(|&index| {
            tickets.iter().all(|ticket| {
                let num = ticket[index];
                return test_num_and_field_rule(field_rule, &num);
            })
        })
        .collect();

    return matching_index;
}

#[aoc(day16, part2)]
fn solve_part2(input: &InputRules) -> usize {
    let mut valid_tickets: Vec<Vec<usize>> = vec![];

    input.tickets.iter().for_each(|ticket| {
        if ticket.iter().all(|num| test_num(input, num)) {
            valid_tickets.push(ticket.clone());
        }
    });

    let mut unsolved_fields: Vec<FieldRule> = input.field_rules.clone();

    let mut result = 1;
    while unsolved_fields.len() > 0 {
        let field = unsolved_fields.pop().unwrap();

        let matching_index = match_field(&valid_tickets, &field);

        if matching_index.len() == 1 {
            let index = matching_index.first().unwrap();

            if field.label.contains("departure") {
                result *= valid_tickets.last().unwrap()[*index];
            }
            valid_tickets = valid_tickets
                .iter()
                .map(|ticket| {
                    let mut clone = ticket.clone();
                    clone.remove(*index);
                    clone
                })
                .collect();
        } else {
            unsolved_fields.insert(0, field);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50
your ticket:
7,1,14
nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 71;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let real_input = r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19
your ticket:
11,12,13
nearby tickets:
3,9,18
15,1,5
5,14,9
";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 1;
        assert_eq!(result, expected);
    }
}
