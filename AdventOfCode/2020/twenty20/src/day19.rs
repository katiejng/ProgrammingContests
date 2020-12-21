#[aoc_generator(day19)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| String::from(l)).collect()
}

#[aoc(day19, part1)]
fn solve_part1(_input: &[String]) -> usize {
    1
}

#[aoc(day19, part2)]
fn solve_part2(_input: &[String]) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"1";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let real_input = r"1";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 1;
        assert_eq!(result, expected);
    }
}
