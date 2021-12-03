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

#[aoc(day1, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut answer: usize = 0;
    for number1 in 0..input.len() - 3 {
        let w1 = input[number1] + input[number1 + 1] + input[number1 + 2];
        let w2 = input[number1 + 1] + input[number1 + 2] + input[number1 + 3];
        if w2 > w1 {
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
    #[test]
    fn part2_test_2() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = solve_part2(&input);
        let expected = 5;
        assert_eq!(result, expected);
    }
}
