#[derive(Debug)]
pub enum Direction {
    Up(usize),
    Down(usize),
    Forward(usize),
}
impl Direction {
    pub fn is_up(&self) -> bool {
        match *self {
            Direction::Up(_) => true,
            _ => false,
        }
    }
    pub fn is_down(&self) -> bool {
        match *self {
            Direction::Down(_) => true,
            _ => false,
        }
    }
    pub fn is_forward(&self) -> bool {
        match *self {
            Direction::Forward(_) => true,
            _ => false,
        }
    }
    pub fn get_value(&self) -> usize {
        match *self {
            Direction::Up(value) => value,
            Direction::Forward(value) => value,
            Direction::Down(value) => value,
        }
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|l| -> Direction {
            let mut split_line = l.split(" ");
            let command = String::from(split_line.next().unwrap());
            let value = split_line.next().unwrap().parse().unwrap();
            let direction: Direction = match command.as_str() {
                "forward" => Direction::Forward(value),
                "down" => Direction::Down(value),
                "up" => Direction::Up(value),
                _ => Direction::Up(0),
            };
            return direction;
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Direction]) -> usize {
    let mut depth: usize = 0;
    let mut distance: usize = 0;

    input.iter().for_each(|direction| {
        if direction.is_up() {
            depth = depth - direction.get_value();
        } else if direction.is_down() {
            depth = depth + direction.get_value();
        } else {
            distance = distance + direction.get_value();
        }
    });
    return depth * distance;
}
#[aoc(day2, part2)]
pub fn solve_part2(input: &[Direction]) -> usize {
    let mut aim = 0;
    let mut depth: usize = 0;
    let mut distance: usize = 0;

    input.iter().for_each(|direction| {
        if direction.is_up() {
            aim = aim - direction.get_value();
        } else if direction.is_down() {
            aim = aim + direction.get_value();
        } else {
            depth = depth + (aim * direction.get_value());
            distance = distance + direction.get_value();
        }
    });
    return depth * distance;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let parsed_input = input_generator(input);
        let result = solve_part1(&parsed_input);
        let expected = 150;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let parsed_input = input_generator(input);
        let result = solve_part2(&parsed_input);
        let expected = 900;
        assert_eq!(result, expected);
    }
}
