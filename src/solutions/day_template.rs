use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use crate::solver::Solver;

pub struct Solution;

impl Solver for Solution {
    type Input = Vec<String>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input(&self, file: File) -> Self::Input {
        BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        panic!("Not implemented!");
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        panic!("Not implemented!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
    }

    #[test]
    fn test_second() {
    }
}
