enum XYZ {
    X,
    Y,
    Z,
}

enum ABC {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn vs(self, opponent: HandShape) -> GameResult {
        if self == opponent {
            return GameResult::Tie;
        }

        let beats = self.beats();
        if opponent == beats {
            return GameResult::Win;
        } else {
            return GameResult::Lose;
        }
    }

    fn beats(self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Scissors,
            HandShape::Paper => HandShape::Rock,
            HandShape::Scissors => HandShape::Paper,
        }
    }

    fn loses_to(self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Paper,
            HandShape::Paper => HandShape::Scissors,
            HandShape::Scissors => HandShape::Rock,
        }
    }

    fn score(self) -> usize {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameResult {
    Win,
    Tie,
    Lose,
}

impl GameResult {
    fn score(self) -> usize {
        match self {
            GameResult::Lose => return 0,
            GameResult::Win => return 6,
            GameResult::Tie => return 3,
        }
    }
}

pub struct InputShape {
    first_column: ABC,
    second_column: XYZ,
}

pub struct GameSummary {
    my_move: HandShape,
    score: usize,
}

impl InputShape {
    pub fn get_part_one(&self) -> GameSummary {
        let elf_move = match self.first_column {
            ABC::A => HandShape::Rock,
            ABC::B => HandShape::Paper,
            ABC::C => HandShape::Scissors,
        };

        let my_move: HandShape = match self.second_column {
            XYZ::X => HandShape::Rock,
            XYZ::Y => HandShape::Paper,
            XYZ::Z => HandShape::Scissors,
        };

        let game_result = my_move.vs(elf_move);

        let game_score = game_result.score();

        return GameSummary {
            my_move: my_move,
            score: game_score,
        };
    }

    pub fn get_part_two(&self) -> GameSummary {
        let elf_move = match self.first_column {
            ABC::A => HandShape::Rock,
            ABC::B => HandShape::Paper,
            ABC::C => HandShape::Scissors,
        };

        let game_result: GameResult = match self.second_column {
            XYZ::X => GameResult::Lose,
            XYZ::Y => GameResult::Tie,
            XYZ::Z => GameResult::Win,
        };

        let my_move = match game_result {
            GameResult::Win => elf_move.loses_to(),
            GameResult::Tie => elf_move,
            GameResult::Lose => elf_move.beats(),
        };

        let score = game_result.score();

        return GameSummary { my_move, score };
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<InputShape> {
    input
        .lines()
        .map(|l| {
            let moves: Vec<&str> = l.split_whitespace().collect();
            InputShape {
                first_column: match moves[0] {
                    "A" => ABC::A,
                    "B" => ABC::B,
                    "C" => ABC::C,
                    _ => ABC::A,
                },
                second_column: match moves[1] {
                    "X" => XYZ::X,
                    "Y" => XYZ::Y,
                    "Z" => XYZ::Z,
                    _ => XYZ::X,
                },
            }
        })
        .collect::<Vec<InputShape>>()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[InputShape]) -> usize {
    let results = input.iter().map(|input_row| input_row.get_part_one());

    return results
        .map(|game_summary| game_summary.score + game_summary.my_move.score())
        .sum();
}
#[aoc(day2, part2)]
pub fn solve_part2(input: &[InputShape]) -> usize {
    let results = input.iter().map(|input_row| input_row.get_part_two());

    return results
        .map(|game_summary| game_summary.score + game_summary.my_move.score())
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_test_1() {
        let parsed_input = input_generator(&TEST_INPUT);
        let result = solve_part1(&parsed_input);
        let expected = 15;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_1() {
        let parsed_input = input_generator(&TEST_INPUT);
        let result = solve_part2(&parsed_input);
        let expected = 12;
        assert_eq!(result, expected);
    }
}
