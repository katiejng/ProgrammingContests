#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.trim().chars().filter(|&c| c != ' ').collect())
        .collect()
}

fn solve_simple(simple_problem: Vec<String>) -> usize {
    println!("slice: {:?}", simple_problem);
    let mut result = 0;
    let mut state: &String = &'+'.to_string();

    simple_problem.iter().for_each(|c| match c.as_str() {
        "*" => state = c,
        "+" => state = c,
        _ => {
            let num: usize = c.to_string().parse().unwrap();
            match state.as_str() {
                "*" => result *= num,
                "+" => result += num,
                _ => {}
            }
        }
    });
    result
}

fn solve_ordered(simple_problem: Vec<String>) -> usize {
    let mut i = 0;
    let mut simple_problem = simple_problem.clone();

    while i < simple_problem.len() {
        if simple_problem[i] == '+'.to_string() {
            let a: usize = simple_problem[i - 1].parse().unwrap();
            let b: usize = simple_problem[i + 1].parse().unwrap();
            let solution: usize = a + b;

            simple_problem.drain(i - 1..=i + 1);
            simple_problem.insert(i - 1, solution.to_string());
            i = i - 1;
        } else {
            i += 1;
        }
    }

    let result = solve_simple(simple_problem);
    return result;
}

fn solve(problem: &Vec<char>, ordered: bool) -> usize {
    // solve brackets

    let mut problem: Vec<String> = problem.iter().map(|c| c.to_string()).collect();
    let mut i = 0;

    let mut open_bracket_tracker: Vec<usize> = vec![];

    while i < problem.len() {
        if problem[i] == '('.to_string() {
            // found open bracket
            open_bracket_tracker.push(i);
            i += 1;
        } else if problem[i] == ')'.to_string() {
            // close a bracket
            let open_index = open_bracket_tracker.pop().unwrap();
            let solved = if ordered {
                solve_ordered(problem[open_index + 1..i].to_vec())
            } else {
                solve_simple(problem[open_index + 1..i].to_vec())
            };
            problem.drain(open_index..=i);
            problem.insert(open_index, solved.to_string());
            i = open_index;
        } else {
            i += 1;
        }
    }

    return if ordered {
        solve_ordered(problem)
    } else {
        solve_simple(problem)
    };
}

#[aoc(day18, part1)]
fn solve_part1(input: &[Vec<char>]) -> usize {
    let result: usize = input.iter().map(|problem| solve(problem, false)).sum();
    result
}

#[aoc(day18, part2)]
fn solve_part2(input: &[Vec<char>]) -> usize {
    let result: usize = input.iter().map(|problem| solve(problem, true)).sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 71 + 51;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let real_input = r"2 * 3 + (4 * 5)";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 46;
        assert_eq!(result, expected);
    }
}
