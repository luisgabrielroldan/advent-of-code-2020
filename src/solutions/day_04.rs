use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use crate::solver::Solver;

pub struct Solution;

impl Solver for Solution {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Self::Input {
        BufReader::new(file).lines().map(|l| l.unwrap()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        parse_passports(input)
            .iter()
            .filter(|p| has_required_fields(p))
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        parse_passports(input)
            .iter()
            .filter(|p| has_required_fields(p))
            .filter(|p| has_valid_fields(p))
            .count()
    }
}

fn parse_passports(input: &Vec<String>) -> Vec<HashMap<&str, &str>> {
    let mut passports: Vec<HashMap<&str, &str>> = Vec::new();
    let mut passport: HashMap<&str, &str> = HashMap::new();

    for line in input {
        if line == "" {
            passports.push(passport);
            passport = HashMap::new();
            continue;
        }

        let fields: Vec<&str> = line.split(' ').collect();

        for field in fields {
            let parts: Vec<&str> = field.split(':').collect();
            passport.insert(parts[0], parts[1]);
        }
    }

    passports.push(passport);

    passports
}

fn has_required_fields(passport: &HashMap<&str, &str>) -> bool {
    let required = vec![
        "byr", /*"cid",*/ "ecl", "eyr", "hcl", "hgt", "iyr", "pid",
    ];
    let mut fields: Vec<&&str> = passport.keys().collect();

    fields.sort();

    required.iter().all(|f| fields.contains(&f))
}

fn has_valid_fields(passport: &HashMap<&str, &str>) -> bool {
    lazy_static! {
        static ref RE_4DIGITS: Regex = Regex::new(r"^\d{4}$").unwrap();
        static ref RE_HGT: Regex = Regex::new(r"^(\d{2,3})(cm|in)?$").unwrap();
        static ref RE_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref RE_PASSID: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    for (field, value) in passport {
        if !match *field {
            "byr" => {
                RE_4DIGITS.is_match(value) && (1920..=2002).contains(&(value.parse().unwrap()))
            }
            "iyr" => {
                RE_4DIGITS.is_match(value) && (2010..=2020).contains(&(value.parse().unwrap()))
            }
            "eyr" => {
                RE_4DIGITS.is_match(value) && (2020..=2030).contains(&(value.parse().unwrap()))
            }
            "hgt" => {
                if let Some(capture) = RE_HGT.captures(value) {
                    let height = if let Some(m) = capture.get(1) {
                        m.as_str().parse::<u32>().unwrap()
                    } else {
                        0
                    };

                    let unit = if let Some(m) = capture.get(2) {
                        m.as_str()
                    } else {
                        ""
                    };

                    match (height, unit) {
                        (150..=193, "cm") => true,
                        (59..=76, "in") => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            "hcl" => RE_HCL.is_match(value),
            "ecl" => {
                vec!["amb","blu", "brn", "gry", "grn", "hzl", "oth"].contains(value)
            }
            "pid" => {
                RE_PASSID.is_match(value)
            }
            _ => true,
        } {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect()
    }

    #[test]
    fn test_first() {
        let input = input();
        let count = Solution {}.solve_first(&input);

        assert!(count == 2);
    }

    #[test]
    fn test_second() {
        let input = input();
        let count = Solution {}.solve_second(&input);

        assert!(count == 2);
    }
}
