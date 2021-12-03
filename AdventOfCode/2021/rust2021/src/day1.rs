#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            let entry = l.trim().parse().unwrap();
            entry
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let mut answer: usize = 0;
    for number1 in 0..input.len() - 1 {
        if input[number1 + 1] > input[number1] {
            answer = answer + 1;
        }
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test_1() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = solve_part1(&input);
        let expected = 7;
        assert_eq!(result, expected);
    }
}
