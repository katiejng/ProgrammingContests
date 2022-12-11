pub struct Pair {
    a_start: usize,
    a_end: usize,
    b_start: usize,
    b_end: usize,
}

impl Pair {
    fn fully_contains(&self) -> bool {
        self.a_start <= self.b_start && self.a_end >= self.b_end
            || self.b_start <= self.a_start && self.b_end >= self.a_end
    }

    fn no_overlap(&self) -> bool {
        self.b_start > self.a_end || self.a_start > self.b_end
    }

    fn some_overlap(&self) -> bool {
        return !self.no_overlap();
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(',').collect();

            let split_one: Vec<usize> = split
                .into_iter()
                .flat_map(|pair| {
                    pair.split('-')
                        .map(|num| num.trim().parse::<usize>().unwrap())
                })
                .collect();

            Pair {
                a_start: split_one[0],
                a_end: split_one[1],
                b_start: split_one[2],
                b_end: split_one[3],
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Pair]) -> usize {
    input.iter().filter(|pair| pair.fully_contains()).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Pair]) -> usize {
    input.iter().filter(|pair| pair.some_overlap()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_test_1() {
        let parsed_input = input_generator(TEST_INPUT);
        let result = solve_part1(&parsed_input);
        let expected = 2;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_1() {
        let parsed_input = input_generator(TEST_INPUT);
        let result = solve_part2(&parsed_input);
        let expected = 4;
        assert_eq!(result, expected);
    }
}
