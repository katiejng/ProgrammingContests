#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| -> String {
            return String::from(l.trim());
        })
        .collect()
}

fn get_most_frequent(input: &[String], index: usize, get_most: bool) -> char {
    let count = input
        .iter()
        .filter(|item| {
            return item.chars().nth(index).unwrap() == '0';
        })
        .count();

    if count * 2 == input.len() {
        if get_most {
            return '1';
        }
        return '0';
    }
    let is_greater = count > input.len() / 2;
    if get_most && is_greater || (!get_most && !is_greater) {
        return '0';
    } else {
        return '1';
    }
}

fn get_most_frequent_array(input: &[String], get_most: bool) -> Vec<String> {
    let mut min_string: Vec<String> = vec![];
    for i in 0..input[0].len() {
        min_string.push(get_most_frequent(input, i, get_most).to_string());
    }
    return min_string;
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> isize {
    let min_string: Vec<String> = get_most_frequent_array(input, true);
    let gamma = isize::from_str_radix(min_string.join("").as_str(), 2).unwrap();

    let max_string = get_most_frequent_array(input, false);
    let epsilon = isize::from_str_radix(max_string.join("").as_str(), 2).unwrap();

    return gamma * epsilon;
}

pub fn filter_2(input: &[String], get_most: bool) -> isize {
    let mut sample: Vec<String> = input.to_vec();
    let mut index = 0;
    while sample.len() > 1 {
        let chosen = get_most_frequent(&sample, index, get_most);
        sample = sample
            .iter()
            .filter(|item| return item.chars().nth(index).unwrap() == chosen)
            .map(|item| item.to_string())
            .collect();

        index = index + 1;
    }

    let oxygen = isize::from_str_radix(&sample[0], 2).unwrap();

    return oxygen;
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> isize {
    let oxygen = filter_2(input, true);
    let co2 = filter_2(input, false);
    return oxygen * co2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_1() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let parsed_input = input_generator(input);
        let result = solve_part1(&parsed_input);
        let expected = 198;
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test_2() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let parsed_input = input_generator(input);
        let result = solve_part2(&parsed_input);
        let expected = 230;
        assert_eq!(result, expected);
    }
}
