#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

#[aoc(day3, part1)]
fn solve_part1(input: &[Vec<char>]) -> usize {
    let down = 1;
    let right = 3;

    solve_helper(input, down, right)
}

fn solve_helper(input: &[Vec<char>], down: usize, right: usize) -> usize {
    let width = input[0].len();
    let steps = input.len() / down;
    let mut result = 0;
    for i in 1..steps {
        let x = i * right % width;
        let y = i * down;
        if input[y][x] == '#' {
            result += 1;
        }
    }
    return result;
}
#[aoc(day3, part2)]
fn solve_part2(input: &[Vec<char>]) -> usize {
    let tests = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    tests
        .iter()
        .map(|test| solve_helper(input, test.1, test.0))
        .product()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"..##.......
                        #...#...#..
                        .#....#..#.
                        ..#.#...#.#
                        .#...##..#.
                        ..#.##.....
                        .#.#.#....#
                        .#........#
                        #.##...#...
                        #...##....#
                        .#..#...#.#";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 7;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test() {
        let real_input = r"..##.......
                        #...#...#..
                        .#....#..#.
                        ..#.#...#.#
                        .#...##..#.
                        ..#.##.....
                        .#.#.#....#
                        .#........#
                        #.##...#...
                        #...##....#
                        .#..#...#.#";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 336;
        assert_eq!(result, expected);
    }
}
