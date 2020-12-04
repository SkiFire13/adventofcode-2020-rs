#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<HashMap<&'a str, &'a str>>;

pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|s| s.split(':').collect_tuple().expect("Invalid input"))
                .collect()
        })
        .collect()
}

fn has_required_fields<'a>(passport: &&HashMap<&'a str, &'a str>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|key| passport.contains_key(key))
}

pub fn part1(input: &Input) -> usize {
    let passports = input;
    passports.iter().filter(has_required_fields).count()
}

pub fn part2(input: &Input) -> usize {
    let passports = input;
    passports
        .iter()
        .filter(|passport| {
            has_required_fields(passport)
                && matches!(passport[&"byr"].parse(), Ok(1920..=2002))
                && matches!(passport[&"iyr"].parse(), Ok(2010..=2020))
                && matches!(passport[&"eyr"].parse(), Ok(2020..=2030))
                && passport[&"hgt"].split(&['c', 'i'][..]).collect_tuple()
                    .map(|(n, u)| matches!((n.parse(), u), (Ok(150..=193), "m") | (Ok(59..=76), "n")))
                    .unwrap_or(false)
                && passport[&"hcl"].len() == 7
                && passport[&"hcl"].starts_with('#')
                && passport[&"hcl"][1..].chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
                && matches!(passport[&"ecl"], "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
                && passport[&"pid"].len() == 9
                && passport[&"pid"].chars().all(|c| matches!(c, '0'..='9'))
        })
        .count()
}
