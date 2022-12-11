pub struct Rucksack {
    items: Vec<Supply>,
    midpoint: usize,
}

impl Rucksack {
    fn new(items: Vec<Supply>) -> Self {
        let midpoint = items.len() / 2;
        Self { items, midpoint }
    }
    fn first_compartment(&self) -> &[Supply] {
        &self.items[0..self.midpoint]
    }

    fn second_compartment(&self) -> &[Supply] {
        &self.items[self.midpoint..]
    }

    fn in_both_compartments(&self) -> &Supply {
        let a = self.first_compartment().into_iter().find(|first| {
            let b = self
                .second_compartment()
                .into_iter()
                .find(|second| first.item == second.item);
            return b.is_some();
        });

        return a.unwrap();
    }

    fn inter(&self, other: &[Supply]) -> Vec<Supply> {
        let a = self
            .items
            .clone()
            .into_iter()
            .filter(|&first| {
                let b = other.iter().find(|second| first.item == second.item);
                return b.is_some();
            })
            .to_owned()
            .collect();

        return a;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Supply {
    item: char,
}

impl Supply {
    fn get_item_value(&self) -> usize {
        if self.item >= 'a' && self.item <= 'z' {
            return (self.item as u32 - 'a' as u32 + 1) as usize; // 'a' - 'a' + 1
        }
        return (self.item as u32 - 'A' as u32 + 27) as usize; // 'A' - 'A' + 26 + 1
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|l| Rucksack::new(l.chars().map(|i| Supply { item: i }).collect()))
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Rucksack]) -> usize {
    let result = input.iter().map(|ruck| ruck.in_both_compartments());

    result.map(|item| item.get_item_value()).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Rucksack]) -> usize {
    let loops = input.len() / 3;
    let result = (0..loops).map(|iter| {
        let r1 = &input[iter * 3];
        let r2 = &input[iter * 3 + 1];
        let r3 = &input[iter * 3 + 2];

        let inter1 = r1.inter(&r2.items);
        let inter2 = r3.inter(&inter1);
        return inter2[0];
    });

    result.map(|supply| supply.get_item_value()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn part1_test_1() {
        let parsed_input = input_generator(&TEST_INPUT);
        let result = solve_part1(&parsed_input);
        let expected = 157;
        assert_eq!(result, expected);
    }
    #[test]
    fn part1_test_2() {
        let parsed_input = input_generator(&TEST_INPUT);
        let result = solve_part2(&parsed_input);
        let expected = 70;
        assert_eq!(result, expected);
    }
}
