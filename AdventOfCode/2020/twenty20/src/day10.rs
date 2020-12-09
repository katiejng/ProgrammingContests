#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn solve_part1(input: &[usize]) -> usize {
    return 1;
}

#[aoc(day10, part2)]
fn solve_part2(input: &[usize]) -> usize {
    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let real_input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 1;
        assert_eq!(result, expected);
    }
}
