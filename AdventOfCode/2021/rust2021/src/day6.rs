#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|item| item.trim().parse().unwrap())
        .collect()
}

fn do_it(input: &[usize], days: usize) -> usize {
    let mut buckets = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    input.iter().for_each(|item| buckets[*item] += 1);

    (0..days).for_each(|_| {
        let babies = buckets[0];
        (1..9).for_each(|i| buckets[i - 1] = buckets[i]);
        buckets[6] += babies;
        buckets[8] = babies;
    });
    buckets.iter().sum()
}
#[aoc(day6, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    do_it(input, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    do_it(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_1() {
        let input = "3,4,3,1,2";
        let parsed_input = input_generator(input);
        let result = solve_part1(&parsed_input);
        let expected = 5934;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_1() {
        let input = "3,4,3,1,2";
        let parsed_input = input_generator(input);
        let result = solve_part2(&parsed_input);
        let expected = 26984457539;
        assert_eq!(result, expected);
    }
}

/*
   0: 34312
   7: 34312 (12334 (sorted)) +2 34556
   14: 3431234556 (sorted + 2) (sorted = 1233344556) (add 2 3455566778)
   21:
*/
