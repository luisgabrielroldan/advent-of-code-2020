use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use crate::solver::Solver;

pub struct Solution;

// Delta = (dx, dy)
type Delta = (usize, usize);

impl Solver for Solution {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Self::Input {
        BufReader::new(file).lines().map(|l| l.unwrap()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let map = Map::new(input);
        map.count_trees((3, 1))
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let map = Map::new(input);
        let a = map.count_trees((1, 1));
        let b = map.count_trees((3, 1));
        let c = map.count_trees((5, 1));
        let d = map.count_trees((7, 1));
        let e = map.count_trees((1, 2));

        a * b * c * d * e
    }
}

struct Map {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Map {
    pub fn new(lines: &Vec<String>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let mut cells: Vec<bool> = vec![false; width * height];
        let mut i = 0;

        for l in lines {
            for c in l.chars() {
                cells[i] = c == '#';
                i += 1;
            }
        }

        Self {
            width: width,
            height: height,
            cells: cells,
        }
    }

    fn count_trees(&self, mov: Delta) -> usize {
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;

        let (dx, dy) = mov;

        while y < self.height {
            if self.get(x, y) {
                count += 1
            }

            x += dx;
            y += dy;
        }

        count
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + (x % self.width)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect()
    }

    #[test]
    fn test_first() {
        let input = input();
        let map = Map::new(&input);
        let trees = map.count_trees((3, 1));

        assert!(trees == 7);
    }

    #[test]
    fn test_second() {
        let input = input();
        let map = Map::new(&input);

        assert!(map.count_trees((1, 1)) == 2);
        assert!(map.count_trees((3, 1)) == 7);
        assert!(map.count_trees((5, 1)) == 3);
        assert!(map.count_trees((7, 1)) == 4);
        assert!(map.count_trees((1, 2)) == 2);
    }
}
