#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn get_unique_string(input: &[char], string_len: usize) -> usize {
    let mut start_index = 0;
    let mut end_index = 1;

    loop {
        let has_overlap = (start_index..end_index)
            .find(|&index| input[end_index] == input[index])
            .is_some();

        if !has_overlap {
            if end_index == start_index + string_len - 1 {
                return end_index + 1;
            }
            end_index += 1;
        } else {
            start_index += 1;
            end_index = start_index + 1;
        }

        if end_index >= input.len() {
            return 0;
        }
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[char]) -> usize {
    get_unique_string(input, 4)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[char]) -> usize {
    get_unique_string(input, 14)
}

#[cfg(test)]
mod tests {

    use super::*;
    const TEST_INPUTS: &[&str] = &[
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn part1_test_1() {
        const TEST_RESULTS: &[usize] = &[7, 5, 6, 10, 11];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part1(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }

    #[test]
    fn part2_test_1() {
        const TEST_RESULTS: &[usize] = &[19, 23, 23, 29, 26];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part2(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }
}
