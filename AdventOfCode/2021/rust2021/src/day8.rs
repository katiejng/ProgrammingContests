pub struct Digit {
    digit: String,
}
pub struct Entry {
    output: Vec<Digit>,
}

impl Digit {
    pub fn get_len(&self) -> usize {
        self.digit.len()
    }

    pub fn is_known(&self) -> bool {
        let known_letters = vec![2, 3, 4, 7];
        known_letters.contains(&self.get_len())
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|l| {
            let mut split_line = l.split('|');
            let _ = split_line.next();

            let output = split_line
                .next()
                .unwrap()
                .split_whitespace()
                .map(|a_digit| Digit {
                    digit: a_digit.to_string(),
                })
                .collect();

            Entry { output }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Entry]) -> usize {
    input
        .iter()
        .flat_map(|entry| entry.output.iter().filter(|o| o.is_known()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_1() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce
";
        let parsed_input = input_generator(input);
        let result = solve_part1(&parsed_input);
        let expected = 26;
        assert_eq!(result, expected);
    }
}
