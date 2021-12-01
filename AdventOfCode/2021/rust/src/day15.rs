use std::collections::HashMap;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
fn solve_part1(input: &[usize]) -> usize {
    let mut last: usize = input.iter().last().unwrap().clone();
    let mut last_seen_map: HashMap<usize, usize> = HashMap::new();
    input.iter().enumerate().for_each(|(index, &value)| {
        last_seen_map.insert(value, index);
    });

    last_seen_map.remove(&last);

    let mut index = input.len() - 1;
    loop {
        let new_last: usize;
        if index == 2019 {
            break;
        }

        if last_seen_map.contains_key(&last) {
            new_last = index - *last_seen_map.get(&last).unwrap();
        } else {
            new_last = 0;
        }
        last_seen_map.insert(last, index);

        last = new_last;
        index += 1;
    }

    return last;
}

#[aoc(day15, part2)]
fn solve_part2(input: &[usize]) -> usize {
    let mut last: usize = input.iter().last().unwrap().clone();
    let mut last_seen_map: HashMap<usize, usize> = HashMap::new();
    input.iter().enumerate().for_each(|(index, &value)| {
        last_seen_map.insert(value, index);
    });

    last_seen_map.remove(&last);

    let mut index = input.len() - 1;
    loop {
        let new_last: usize;
        if index == 30000000 - 1 {
            break;
        }

        if last_seen_map.contains_key(&last) {
            new_last = index - *last_seen_map.get(&last).unwrap();
        } else {
            new_last = 0;
        }
        last_seen_map.insert(last, index);

        last = new_last;
        index += 1;
    }

    return last;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"0,3,6";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 436;
        assert_eq!(result, expected);
    }
}
