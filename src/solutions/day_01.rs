use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use crate::solver::Solver;

pub struct Solution;

impl Solver for Solution {
    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input(&self, file: File) -> Self::Input {
        BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.parse::<u32>().unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        for x in input {
            for y in input {
                if x + y == 2020 {
                    return x * y;
                }
            }
        }

        panic!("Not found!");
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        for x in input {
            for y in input {
                for z in input {
                    if x + y + z == 2020 {
                        return x * y * z;
                    }
                }
            }
        }

        panic!("Not found!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<u32> {
        vec![1721, 979, 366, 299, 675, 1456]
    }

    #[test]
    fn test_first() {
        let input = input();
        let result = Solution {}.solve_first(&input);

        assert!(result == 514579);
    }

    #[test]
    fn test_second() {
        let input = input();
        let result = Solution {}.solve_second(&input);

        assert!(result == 241861950);
    }
}
