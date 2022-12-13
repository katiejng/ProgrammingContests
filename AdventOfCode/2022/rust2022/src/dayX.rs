#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<usize> {
    vec![1]
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    1
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[usize]) -> isize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUTS: &[&str] = &["Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"];

    #[test]
    fn part1_test_1() {
        const TEST_RESULTS: &[usize] = &[31];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part1(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }

    #[test]
    fn part2_test_1() {
        const TEST_RESULTS: &[isize] = &[29];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part2(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }
}
