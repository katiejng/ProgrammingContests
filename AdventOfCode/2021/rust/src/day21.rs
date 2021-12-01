use std::{
    collections::{HashMap, HashSet},
    vec,
};

#[derive(Debug, Clone)]
struct InputLine {
    allergens: Vec<String>,
    ingredients: HashSet<String>,
}

#[aoc_generator(day21)]
fn input_generator(input: &str) -> Vec<InputLine> {
    input
        .lines()
        .map(|l| {
            let mut test = l.split(" (contains ");
            let a = test
                .next()
                .unwrap()
                .split(" ")
                .map(|word| String::from(word.trim()))
                .collect();

            let b: Vec<String> = match test.next() {
                Some(allergens) => allergens
                    .split(")")
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|word| String::from(word.trim()))
                    .collect(),
                None => {
                    vec![]
                }
            };

            return InputLine {
                ingredients: a,
                allergens: b,
            };
        })
        .collect()
}

#[aoc(day21, part1)]
fn solve_part1(input: &[InputLine]) -> usize {
    let mut results: HashMap<&String, HashSet<String>> = HashMap::new();
    let mut all_ingredients: HashSet<String> = HashSet::new();

    input.iter().for_each(|input_line| {
        all_ingredients.extend(input_line.ingredients.clone());
        input_line.allergens.iter().for_each(|allergen| {
            let allergen_list = results.get(allergen);
            match allergen_list {
                Some(ingredients) => {
                    let mix = ingredients
                        .intersection(&input_line.ingredients)
                        .map(|item| item.clone())
                        .collect();
                    results.insert(allergen, mix);
                }
                None => {
                    results.insert(allergen, input_line.ingredients.clone());
                }
            }
        })
    });

    let mut confirmed_ingredients: HashSet<String> = HashSet::new();

    let mut results: Vec<Vec<&String>> = results
        .iter()
        .map(|item| item.1.iter().map(|ingre| ingre).collect())
        .collect();

    while results.iter().any(|item| item.len() == 1) {
        let result = *results
            .iter()
            .find(|item| item.len() == 1)
            .unwrap()
            .first()
            .unwrap();
        confirmed_ingredients.insert(result.clone());
        results = results
            .iter()
            .map(|alist| {
                let rr: Vec<&String> = alist
                    .iter()
                    .filter(|&&aitem| aitem != result)
                    .map(|&aitem| aitem)
                    .collect();
                return rr;
            })
            .collect();
    }

    let non_ingredients: HashSet<String> = all_ingredients
        .difference(&confirmed_ingredients)
        .map(|item| item.clone())
        .collect();

    let mut sum = 0;

    input
        .iter()
        .for_each(|line| sum += non_ingredients.intersection(&line.ingredients).count());

    return sum;
}

#[aoc(day21, part2)]
fn solve_part2(input: &[InputLine]) -> String {
    let mut results: HashMap<&String, HashSet<String>> = HashMap::new();
    let mut all_ingredients: HashSet<String> = HashSet::new();

    input.iter().for_each(|input_line| {
        all_ingredients.extend(input_line.ingredients.clone());
        input_line.allergens.iter().for_each(|allergen| {
            let allergen_list = results.get(allergen);
            match allergen_list {
                Some(ingredients) => {
                    let mix = ingredients
                        .intersection(&input_line.ingredients)
                        .map(|item| item.clone())
                        .collect();
                    results.insert(allergen, mix);
                }
                None => {
                    results.insert(allergen, input_line.ingredients.clone());
                }
            }
        })
    });

    let mut confirmed_ingredients: Vec<(String, String)> = vec![];

    let mut results: Vec<(&String, Vec<String>)> = results
        .iter()
        .map(|item| (*item.0, item.1.iter().map(|ingre| ingre.clone()).collect()))
        .collect();

    while results.iter().any(|item| item.1.len() == 1) {
        let result = results.iter().find(|item| item.1.len() == 1).unwrap();
        let ingredient = result.1.first().unwrap();
        confirmed_ingredients.push((result.0.clone(), ingredient.clone()));
        results = results
            .iter()
            .map(|alist| {
                let rr: Vec<String> = alist
                    .1
                    .iter()
                    .filter(|aitem| **aitem != ingredient.clone())
                    .map(|aitem| aitem.clone())
                    .collect();
                return (alist.0, rr);
            })
            .collect();
    }
    confirmed_ingredients.sort_by(|a, b| a.0.cmp(&b.0));
    let string_list: Vec<String> = confirmed_ingredients
        .iter()
        .map(|(_, ingredient)| ingredient.clone())
        .collect();

    return string_list.join(",");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test_1() {
        let real_input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = "mxmxvkd,sqjhc,fvjkl";
        assert_eq!(result, expected);
    }
}
