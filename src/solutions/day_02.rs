use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use crate::solver::Solver;

pub struct Solution;

type Policy = (usize, usize, char);

impl Solver for Solution {
    type Input = Vec<String>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input(&self, file: File) -> Self::Input {
        BufReader::new(file).lines().map(|l| l.unwrap()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let entries: Vec<(Policy, &str)> = input.iter().map(|l| parse_entry(l)).collect();
        let mut valid_passwords: u32 = 0;

        for (policy, password) in entries {
            if is_valid_password_part1(password, policy) {
                valid_passwords += 1
            }
        }

        valid_passwords
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let entries: Vec<(Policy, &str)> = input.iter().map(|l| parse_entry(l)).collect();
        let mut valid_passwords: u32 = 0;

        for (policy, password) in entries {
            if is_valid_password_part2(password, policy) {
                valid_passwords += 1
            }
        }

        valid_passwords
    }
}

fn is_valid_password_part1(password: &str, policy: Policy) -> bool {
    let (min, max, c) = policy;
    let count = password.matches(c).count();

    count >= min && count <= max
}

fn is_valid_password_part2(password: &str, policy: Policy) -> bool {
    let (pos1, pos2, c) = policy;
    let password_chars: Vec<char> = password.chars().collect();

    !(password_chars[pos1 - 1] == c && password_chars[pos2 - 1] == c)
        && (password_chars[pos1 - 1] == c || password_chars[pos2 - 1] == c)
}

fn parse_entry(line: &String) -> (Policy, &str) {
    let parts: Vec<&str> = line.split(": ").collect();
    let password = parts[1];
    let policy_parts: Vec<&str> = parts[0].split(|c| c == '-' || c == ' ').collect();

    let policy: Policy = (
        policy_parts[0].parse::<usize>().unwrap(),
        policy_parts[1].parse::<usize>().unwrap(),
        policy_parts[2].chars().next().unwrap(),
    );

    (policy, password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
            .iter()
            .map(|s| String::from(*s))
            .collect();

        let result = Solution {}.solve_first(&input);

        assert!(result == 2);
    }

    #[test]
    fn test_second() {
        let input = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
            .iter()
            .map(|s| String::from(*s))
            .collect();

        let result = Solution {}.solve_second(&input);

        assert!(result == 1);
    }
}
