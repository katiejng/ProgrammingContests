struct InputLine {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<InputLine> {
    input
        .lines()
        .map(|l| {
            let mut words = l.trim().split(" ");

            let min_to_max = words.next().unwrap();
            let mut min_to_max_split = min_to_max.trim().split("-");

            let min = min_to_max_split.next().unwrap().parse().unwrap();
            let max = min_to_max_split.next().unwrap().parse().unwrap();

            let character = words.next().unwrap().chars().next().unwrap();
            let password = words.next().unwrap();
            InputLine {
                character: character,
                max: max,
                min: min,
                password: String::from(password),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[InputLine]) -> usize {
    input
        .iter()
        .filter(|input_line| {
            let occurrences = input_line.password.matches(input_line.character).count();

            return occurrences <= input_line.max && occurrences >= input_line.min;
        })
        .count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &[InputLine]) -> usize {
    input
        .iter()
        .filter(|input_line| {
            let pos1 = input_line.password.chars().nth(input_line.min - 1).unwrap();
            let pos2 = input_line.password.chars().nth(input_line.max - 1).unwrap();

            pos1 == input_line.character && pos2 != input_line.character
                || pos2 == input_line.character && pos1 != input_line.character
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = [
            InputLine {
                character: 'a',
                max: 3,
                min: 1,
                password: String::from("abcde"),
            },
            InputLine {
                character: 'b',
                max: 3,
                min: 1,
                password: String::from("cdefg"),
            },
            InputLine {
                character: 'c',
                max: 9,
                min: 2,
                password: String::from("ccccccccc"),
            },
        ];
        let result = solve_part1(&input);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let input = [
            InputLine {
                character: 'a',
                max: 3,
                min: 1,
                password: String::from("abcde"),
            },
            InputLine {
                character: 'b',
                max: 3,
                min: 1,
                password: String::from("cdefg"),
            },
            InputLine {
                character: 'c',
                max: 9,
                min: 2,
                password: String::from("ccccccccc"),
            },
        ];
        let result = solve_part2(&input);
        let expected = 1;
        assert_eq!(result, expected);
    }
}
