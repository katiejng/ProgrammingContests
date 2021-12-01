use std::collections::{HashMap, HashSet};
struct InputRule {
    bag_name: String,
    count: usize,
    children: Vec<InputRule>,
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<InputRule> {
    input
        .lines()
        .map(|l| {
            let l = l.split_at(l.len() - 1).0;
            let mut split_line = l.split("bags contain ");
            let parent = split_line.next().unwrap().trim();
            let children = split_line.next().unwrap().trim();
            if children == "no other bags" {
                return InputRule {
                    bag_name: String::from(parent),
                    children: vec![],
                    count: 1,
                };
            }
            let children = children
                .split(",")
                .map(|child| {
                    let mut split_child = child.trim().splitn(2, " ");
                    InputRule {
                        count: split_child.next().unwrap().parse().unwrap(),
                        bag_name: String::from(
                            split_child
                                .next()
                                .unwrap()
                                .split("bag")
                                .next()
                                .unwrap()
                                .trim(),
                        ),
                        children: vec![],
                    }
                })
                .collect();
            return InputRule {
                bag_name: String::from(parent),
                children: children,
                count: 1,
            };
        })
        .collect()
}

#[aoc(day7, part1)]
fn solve_part1(input: &[InputRule]) -> usize {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    input.iter().for_each(|item| {
        item.children.iter().for_each(|child| {
            map.entry(&child.bag_name)
                .or_insert(vec![])
                .push(&item.bag_name);
        })
    });
    let mut result: HashSet<&str> = HashSet::new();
    let mut points = vec!["shiny gold"];
    while points.len() > 0 {
        let next_point = points.pop().unwrap();
        if map.contains_key(next_point) {
            let mut result = map.get(next_point).unwrap().clone();
            points.append(&mut result);
        }
        result.insert(next_point);
    }
    return result.len() - 1;
}

fn helper(map: &HashMap<&str, Vec<&InputRule>>, current: &str) -> usize {
    if map.contains_key(current) {
        return 1 + map
            .get(current)
            .unwrap()
            .iter()
            .map(|bag| bag.count * helper(map, &bag.bag_name))
            .sum::<usize>();
    }
    return 1;
}

#[aoc(day7, part2)]
fn solve_part2(input: &[InputRule]) -> usize {
    let mut map: HashMap<&str, Vec<&InputRule>> = HashMap::new();
    input.iter().for_each(|item| {
        item.children.iter().for_each(|child| {
            map.entry(&item.bag_name).or_insert(vec![]).push(child);
        })
    });
    let result = helper(&map, "shiny gold");
    return result - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let real_input = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 126;
        assert_eq!(result, expected);
    }
    #[test]
    fn part2_test_2() {
        let real_input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 32;
        assert_eq!(result, expected);
    }
}
