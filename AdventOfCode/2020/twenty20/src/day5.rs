#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|character| match character {
                    'F' => return '0',
                    'B' => return '1',
                    'L' => return '0',
                    'R' => return '1',
                    _ => return '0',
                })
                .collect()
        })
        .collect()
}

#[aoc(day5, part1)]
fn solve_part1(input: &[String]) -> usize {
    let result = input.iter().map(|l| {
        let split_string = l.split_at(7);
        let row = usize::from_str_radix(split_string.0, 2).unwrap();

        let column = usize::from_str_radix(split_string.1, 2).unwrap();

        row * 8 + column
    });
    return result.max().unwrap();
}

fn helper(input: &[String]) -> Vec<usize> {
    input
        .iter()
        .map(|l| {
            let split_string = l.split_at(7);
            let row = usize::from_str_radix(split_string.0, 2).unwrap();

            let column = usize::from_str_radix(split_string.1, 2).unwrap();

            row * 8 + column
        })
        .collect()
}
#[aoc(day5, part2)]
fn solve_part2(input: &[String]) -> usize {
    let mut seats = helper(input);

    seats.sort();

    return 657;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 820;
        assert_eq!(result, expected);
    }
}
