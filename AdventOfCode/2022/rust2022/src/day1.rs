#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .into_iter()
        .map(|elf| {
            elf.lines()
                .map(|line| line.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<usize>]) -> usize {
    let mut answer: usize = 0;
    for number1 in 0..input.len() - 1 {
        let sum = input[number1].iter().sum();
        if sum > answer {
            answer = sum;
        }
    }
    return answer;
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<usize>]) -> usize {
    let mut elves: Vec<usize> = input.iter().map(|elf| elf.iter().sum()).collect();

    elves.sort();
    elves.reverse();

    elves[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part2_test_1() {
        let result = solve_part1(&input_generator(&TEST_INPUT));
        let expected = 24000;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_2() {
        let result = solve_part2(&input_generator(&TEST_INPUT));
        let expected = 45000;
        assert_eq!(result, expected);
    }
}
