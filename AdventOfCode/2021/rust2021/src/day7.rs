use std::convert::TryInto;

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|item| item.trim().parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[usize]) -> isize {
    let mut positions = input.to_vec();
    positions.sort();
    if positions.len() % 2 != 0 {
        //odd
        let center = positions.len() / 2 + 1;
        println!("center {}", center);
        let center_value = positions[center];
        println!("center value {} ", center_value);

        return positions
            .iter()
            .map(|&pos| {
                let a: isize = center_value.try_into().unwrap();
                let b: isize = pos.try_into().unwrap();
                let diff: isize = a - b;

                return diff.abs();
            })
            .sum();
    } else {
        //even
        let center = positions.len() / 2;
        println!("center {}", center);
        let center_value_one = positions[center];
        let center_value_two = positions[center + 1];

        return (center_value_one..=center_value_two)
            .map(|center_value| {
                return positions
                    .iter()
                    .map(|&pos| {
                        let a: isize = center_value.try_into().unwrap();
                        let b: isize = pos.try_into().unwrap();
                        let diff: isize = a - b;

                        return diff.abs();
                    })
                    .sum();
            })
            .min()
            .unwrap();
    }
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[usize]) -> isize {
    let mut positions = input.to_vec();
    positions.sort();

    let center_value_one = *positions.first().unwrap();
    let center_value_two = *positions.last().unwrap();

    return (center_value_one..=center_value_two)
        .map(|center_value| {
            return positions
                .iter()
                .map(|&pos| {
                    let a: isize = center_value.try_into().unwrap();
                    let b: isize = pos.try_into().unwrap();
                    let diff: isize = a - b;

                    let abs_diff = diff.abs();
                    return (abs_diff * (abs_diff + 1)) / 2;
                })
                .sum();
        })
        .min()
        .unwrap();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let parsed_input = input_generator(input);
        let result = solve_part1(&parsed_input);
        let expected = 37;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let parsed_input = input_generator(input);
        let result = solve_part2(&parsed_input);
        let expected = 168;
        assert_eq!(result, expected);
    }
}
