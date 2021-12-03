#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| -> String {
            return String::from(l.trim());
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> isize {
    let mut min_string: Vec<String> = vec![];

    for i in 0..input[0].len() {
        let count = input
            .iter()
            .filter(|item| {
                return item.chars().nth(i).unwrap() == '0';
            })
            .count();

        if count > input.len() / 2 {
            min_string.push("0".to_string())
        } else {
            min_string.push("1".to_string())
        }
    }
    let gamma = isize::from_str_radix(min_string.join("").as_str(), 2).unwrap();

    let mut max_string: Vec<String> = vec![];

    for i in 0..input[0].len() {
        let count = input
            .iter()
            .filter(|item| {
                return item.chars().nth(i).unwrap() == '0';
            })
            .count();

        if count < input.len() / 2 {
            max_string.push("0".to_string())
        } else {
            max_string.push("1".to_string())
        }
    }
    let epsilon = isize::from_str_radix(max_string.join("").as_str(), 2).unwrap();

    println!("{}: {}", min_string.join(","), gamma);
    println!("{}: {}", max_string.join(","), epsilon);
    return gamma * epsilon;
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
}
