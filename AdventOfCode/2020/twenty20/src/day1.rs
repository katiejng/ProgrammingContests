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
    for number in 0..input.len() - 1 {
        for number2 in number + 1..input.len() {
            if input[number] + input[number2] == 2020 {
                answer = input[number] * input[number2];
                break;
            }
        }
    }
    return answer;
}

#[aoc(day1, part2, alt1)]
pub fn solve_part2_1(input: &[usize]) -> usize {
    let mut answer: usize = 0;
    for number1 in 0..input.len() - 2 {
        let a = input[number1];
        for number2 in number1 + 1..input.len() - 1 {
            let b = input[number2];
            for number3 in number2 + 1..input.len() {
                let c = input[number3];
                if a + b + c == 2020 {
                    answer = a * b * c;
                    break;
                }
            }
        }
    }
    return answer;
}

#[aoc(day1, part2, alt2)]
pub fn solve_part2_2(input: &[usize]) -> usize {
    let mut result: usize = 0;

    input.iter().for_each(|x| {
        input.iter().for_each(|y| {
            input.iter().for_each(|z| {
                if x + y + z == 2020 {
                    result = x * y * z;
                }
            });
        });
    });
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = [1721, 979, 366, 299, 675, 1456];
        let result = solve_part1(&input);
        let expected = 514579;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_1() {
        let input = [1721, 979, 366, 299, 675, 1456];
        let result = solve_part2_1(&input);
        let expected = 241861950;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_2() {
        let input = [1721, 979, 366, 299, 675, 1456];
        let result = solve_part2_2(&input);
        let expected = 241861950;
        assert_eq!(result, expected);
    }
}
