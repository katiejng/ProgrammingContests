use std::{cmp::Ordering, fmt};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Clone)]
pub struct Pair(Packet, Packet);

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Packet {
    Num(usize),
    Arr(Vec<Packet>),
}

fn compare_array(a: &Vec<Packet>, b: &Vec<Packet>) -> Ordering {
    let mut index = 0;

    loop {
        if index == a.len() && a.len() == b.len() {
            return Ordering::Equal;
        }
        // a has run out early
        if index >= a.len() {
            return Ordering::Less;
        }
        // b run out early
        if index >= b.len() {
            return Ordering::Greater;
        }

        // a was bigger
        if a[index].cmp(&b[index]) == Ordering::Greater {
            return Ordering::Greater;
        }

        if a[index].cmp(&b[index]) == Ordering::Less {
            return Ordering::Less;
        }
        index += 1
    }
}

impl Eq for Packet {}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Num(a_num), Packet::Num(b_num)) => a_num.cmp(b_num),
            (Packet::Num(a_num), Packet::Arr(b_arr)) => {
                let a_arr = vec![Packet::Num(*a_num)];
                compare_array(&a_arr, &b_arr)
            }
            (Packet::Arr(a_arr), Packet::Num(b_num)) => {
                let b_arr = vec![Packet::Num(*b_num)];
                compare_array(&a_arr, &b_arr)
            }
            (Packet::Arr(a_arr), Packet::Arr(b_arr)) => compare_array(&a_arr, &b_arr),
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Num(num) => write!(f, "{}", num),
            Packet::Arr(packet) => write!(f, "{:?}", packet),
        }
    }
}

fn parse_usize(input: &str) -> IResult<&str, Packet> {
    let (input, result) = digit1(input)?;

    let number: usize = result.parse().unwrap();
    let packet = Packet::Num(number);

    return IResult::Ok((input, packet));
}

fn parse_array(input: &str) -> IResult<&str, Packet> {
    let item = alt((parse_usize, parse_array));

    let items = separated_list0(char(','), item);

    let (input, result) = delimited(char('['), items, char(']'))(input)?;

    let packet = Packet::Arr(result);

    return IResult::Ok((input, packet));
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, result) = separated_pair(parse_array, char('\n'), parse_array)(input)?;

    let pair = Pair(result.0, result.1);

    return IResult::Ok((input, pair));
}

fn parse_pair_list(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(tag("\n\n"), parse_pair)(input)
}
#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<Pair> {
    parse_pair_list(input).ok().unwrap().1
}

#[aoc(day13, part1)]
fn solve_part1(input: &[Pair]) -> usize {
    let results: usize = input
        .iter()
        .enumerate()
        .map(|(index, pair)| {
            let result = pair.0.cmp(&pair.1);
            if result == Ordering::Less {
                return index + 1;
            }
            return 0;
        })
        .sum();

    results
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[Pair]) -> usize {
    let mut lines: Vec<Packet> = input
        .iter()
        .flat_map(|pair| vec![pair.0.clone(), pair.1.clone()])
        .collect();

    let two = Packet::Arr(vec![Packet::Arr(vec![Packet::Num(2)])]);
    let six = Packet::Arr(vec![Packet::Arr(vec![Packet::Num(6)])]);

    lines.push(two.clone());
    lines.push(six.clone());

    lines.sort_by(Packet::cmp);

    let two_index = lines.iter().position(|packet| packet == &two).unwrap();
    let six_index = lines.iter().position(|packet| packet == &six).unwrap();

    (two_index + 1) * (six_index + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUTS: &[&str] = &["[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"];

    #[test]
    fn part1_test_1() {
        const TEST_RESULTS: &[usize] = &[13];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part1(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }

    #[test]
    fn part2_test_1() {
        const TEST_RESULTS: &[usize] = &[140];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part2(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }
}
