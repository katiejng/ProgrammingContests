#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn solve_part1(input: &[usize]) -> usize {
    let mut one_diff = 0;
    let mut three_diff = 1;
    let mut sorted_input = input.clone().to_vec();
    sorted_input.sort();
    sorted_input.insert(0, 0);
    for i in 0..sorted_input.len() - 1 {
        let diff = sorted_input[i + 1] - sorted_input[i];
        if diff == 1 {
            one_diff += 1;
        } else if diff == 3 {
            three_diff += 1;
        }
    }

    return one_diff * three_diff;
}

#[aoc(day10, part2)]
fn solve_part2(input: &[usize]) -> usize {
    let mut sorted_input = input.clone().to_vec();
    sorted_input.sort();
    sorted_input.insert(0, 0);

    let steps = sorted_input.len();
    let mut saved_counts = sorted_input.clone();

    saved_counts[steps - 1] = 1;

    for i in (0..steps - 1).rev() {
        let current = sorted_input[i];
        let mut total = 0;
        for j in 1..4 {
            let option1 = i + j;
            if option1 < steps {
                let next = sorted_input[option1];
                if next <= current + 3 {
                    total += saved_counts[option1];
                }
            }
        }
        saved_counts[i] = total;
    }
    return saved_counts[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"16
10
15
5
1
11
7
19
6
12
4
";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 35;
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_test_2() {
        let real_input = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 220;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let real_input = r"16
10
15
5
1
11
7
19
6
12
4
";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 8;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_2() {
        let real_input = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 19208;
        assert_eq!(result, expected);
    }
}
