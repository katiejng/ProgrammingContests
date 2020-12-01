#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            let entry = l.trim().parse().unwrap();
            entry
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    input.iter().map(|&l| l).sum()
}
