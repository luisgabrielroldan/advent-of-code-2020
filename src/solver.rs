use std::{fmt::Display, fs::File};

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn parse_input(&self, file: File) -> Self::Input;
    fn solve_first(&self, input: &Self::Input) -> Self::Output1;
    fn solve_second(&self, input: &Self::Input) -> Self::Output2;

    fn solve(&self, input_file: String) {
        let file = File::open(input_file).expect("error reading input file");
        let input = self.parse_input(file);

        let sol1 = self.solve_first(&input);
        let sol2 = self.solve_second(&input);

        println!("Part 1: {}", sol1);
        println!("Part 2: {}", sol2);
    }
}
