#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Vec<(String, String)>> {
    input
        .split("\n\n")
        .map(|passport| {
            let first_passport: Vec<Vec<(String, String)>> = passport
                .lines()
                .map(|line| {
                    line.split(" ")
                        .map(|entry| {
                            let mut item = entry.split(":");
                            (
                                item.next().unwrap().to_string(),
                                item.next().unwrap().to_string(),
                            )
                        })
                        .collect()
                })
                .collect();

            first_passport.into_iter().flatten().collect()
        })
        .collect()
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Vec<(String, String)>]) -> usize {
    input
        .iter()
        .filter(|passport| {
            let fields: Vec<&(String, String)> =
                passport.iter().filter(|items| items.0 != "cid").collect();
            fields.iter().count() >= 7
        })
        .count()
}

fn validate_range(min: usize, max: usize, value: usize) -> bool {
    return value <= max && value >= min;
}

fn validate_height(size: String) -> bool {
    let value = size.split_at(size.len() - 2).0.parse().unwrap();

    if size.chars().last().unwrap() == 'm' {
        // cm
        return validate_range(150, 193, value);
    }
    return validate_range(59, 76, value);
}

fn validate_hair_color(color: String) -> bool {
    let mut characters = color.chars();
    if characters.clone().count() != 7 {
        return false;
    }
    if characters.next().unwrap() != '#' {
        return false;
    }

    characters.all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))
}

fn validate_eye_color(color: String) -> bool {
    let eye_colors = [
        String::from("amb"),
        String::from("blu"),
        String::from("brn"),
        String::from("gry"),
        String::from("grn"),
        String::from("hzl"),
        String::from("oth"),
    ];

    eye_colors
        .iter()
        .any(|eye_color| eye_color.as_str() == color)
}

fn validate_passport_number(number: String) -> bool {
    let mut characters = number.chars();
    if characters.clone().count() != 9 {
        return false;
    }

    characters.all(|c| (c >= '0' && c <= '9'))
}
fn validate_field(field: &(String, String)) -> bool {
    let value = field.1.clone();
    match field.0.as_str() {
        "byr" => return validate_range(1920, 2002, value.parse().unwrap()),
        "iyr" => return validate_range(2010, 2020, value.parse().unwrap()),
        "eyr" => return validate_range(2020, 2030, value.parse().unwrap()),
        "hgt" => return validate_height(value),
        "hcl" => return validate_hair_color(value),
        "ecl" => return validate_eye_color(value),
        "pid" => return validate_passport_number(value),
        _ => return true,
    }
}
#[aoc(day4, part2)]
fn solve_part2(input: &[Vec<(String, String)>]) -> usize {
    input
        .iter()
        .filter(|passport| {
            let fields: Vec<&(String, String)> =
                passport.iter().filter(|items| items.0 != "cid").collect();
            return fields.iter().count() >= 7
                && fields.iter().all(|field| validate_field(field.clone()));
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let real_input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";
        let transformed_input = input_generator(real_input);
        let result = solve_part1(&transformed_input);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let real_input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test2() {
        let real_input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
        let transformed_input = input_generator(real_input);
        let result = solve_part2(&transformed_input);
        let expected = 4;
        assert_eq!(result, expected);
    }
}
