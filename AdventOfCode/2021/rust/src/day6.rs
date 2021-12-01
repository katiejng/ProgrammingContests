use std::collections::HashMap;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|group| group.trim().lines().map(|l| l.to_string()).collect())
        .collect()
}

#[aoc(day6, part1)]
fn solve_part1(input: &[Vec<String>]) -> usize {
    for number in 0..input.len() {
        for number1 in 0..input[number].len() {
            println!("{}", input[number][number1])
        }
    }

    let res = input
        .iter()
        .map(|answers| {
            let mut map = HashMap::new();

            answers.iter().for_each(|answer| {
                answer.chars().for_each(|character| {
                    let count = map.entry(character).or_insert(0);
                    *count += 1;
                })
            });

            map.len()
        })
        .sum();

    return res;
}

#[aoc(day6, part2)]
fn solve_part2(input: &[Vec<String>]) -> usize {
    for number in 0..input.len() {
        for number1 in 0..input[number].len() {
            println!("{}", input[number][number1])
        }
    }

    let res = input
        .iter()
        .map(|answers| {
            let mut map: HashMap<char, usize> = HashMap::new();

            answers.iter().for_each(|answer| {
                answer.chars().for_each(|character| {
                    let count = map.entry(character).or_insert(0);
                    *count += 1;
                })
            });

            map.len();

            map.values()
                .filter(|value| value == &&answers.len())
                .count()
        })
        .sum();

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"abc

a
b
c

ab
ac

a
a
a
a

b
";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 11;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let real_input = r"abc

a
b
c

ab
ac

a
a
a
a

b
";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 6;
        assert_eq!(result, expected);
    }
}
