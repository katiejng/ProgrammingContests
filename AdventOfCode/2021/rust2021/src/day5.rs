use std::{collections::HashMap, convert::TryInto, vec};

use regex::Regex;

pub struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {})",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    pub fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    pub fn fetch_points(&self) -> Vec<(usize, usize)> {
        if self.is_horizontal() {
            /* ---- */
            if self.x1 > self.x2 {
                return (self.x2..=self.x1).map(|num| (num, self.y1)).collect();
            } else {
                return (self.x1..=self.x2).map(|num| (num, self.y1)).collect();
            }
        } else if self.is_vertical() {
            /* | */
            if self.y1 > self.y2 {
                return (self.y2..=self.y1).map(|num| (self.x1, num)).collect();
            } else {
                return (self.y1..=self.y2).map(|num| (self.x1, num)).collect();
            }
        } else {
            let x_increment: isize = if self.x1 < self.x2 { 1 } else { -1 };
            let y_increment: isize = if self.y1 < self.y2 { 1 } else { -1 };
            let mut results: Vec<(usize, usize)> = vec![];

            let mut x_start: isize = self.x1.try_into().unwrap();
            let x_end: isize = self.x2.try_into().unwrap();
            let mut y_start: isize = self.y1.try_into().unwrap();
            loop {
                results.push((x_start.try_into().unwrap(), y_start.try_into().unwrap()));
                x_start += x_increment;
                y_start += y_increment;
                if x_start - x_increment == x_end {
                    break;
                }
            }
            return results;
        }
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<Line> {
    let re = Regex::new(r"(\d*),(\d*) -> (\d*),(\d*)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            return Line {
                x1: cap[1].parse().unwrap(),
                y1: cap[2].parse().unwrap(),
                x2: cap[3].parse().unwrap(),
                y2: cap[4].parse().unwrap(),
            };
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Line]) -> usize {
    let mut visited_points: HashMap<(usize, usize), usize> = HashMap::new();
    let straight_lines: Vec<&Line> = input
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect();

    straight_lines.iter().for_each(|line| {
        let points = line.fetch_points();

        points.iter().for_each(|point| {
            let item_ref = visited_points.entry(*point).or_insert(0);
            *item_ref += 1;
        });
    });

    visited_points.values().filter(|val| **val > 1).count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Line]) -> usize {
    let mut visited_points: HashMap<(usize, usize), usize> = HashMap::new();

    input.iter().for_each(|line| {
        let points = line.fetch_points();

        points.iter().for_each(|point| {
            let item_ref = visited_points.entry(*point).or_insert(0);
            *item_ref += 1;
        });
    });

    visited_points.values().filter(|val| **val > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let parsed_input = input_generator(input);
        let result = solve_part1(&parsed_input);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let parsed_input = input_generator(input);
        let result = solve_part2(&parsed_input);
        let expected = 12;
        assert_eq!(result, expected);
    }
}
