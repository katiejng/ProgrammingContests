#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}
fn helper(input: &[usize], goal: usize, start_index: usize, end_index: usize) -> bool {
    for number in start_index..end_index {
        for number2 in number + 1..end_index + 1 {
            if input[number] + input[number2] == goal {
                return true;
            }
        }
    }
    return false;
}

#[aoc(day9, part1)]
fn solve_part1(input: &[usize]) -> usize {
    let rolling_number = 25;
    for cur_index in rolling_number..input.len() - 1 {
        let start_index = cur_index - rolling_number;
        let end_index = cur_index - 1;
        if !helper(input, input[cur_index], start_index, end_index) {
            return input[cur_index];
        }
    }
    return 1;
}

#[aoc(day9, part2)]
fn solve_part2(input: &[usize]) -> usize {
    let goal = 104054607;
    let mut start_index: usize = 0;
    let mut end_index: usize = 0;
    let mut sum: isize = 0;
    loop {
        if sum == goal {
            let slice = &input[start_index..end_index];
            return slice.iter().min().unwrap() + slice.iter().max().unwrap();
        }
        if sum < goal {
            end_index += 1;
            if end_index >= input.len() {
                return 1;
            }
            sum += input[end_index - 1] as isize;
        }
        if sum > goal {
            sum -= input[start_index] as isize;
            start_index += 1;
        }
    }
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
