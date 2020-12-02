mod day_01;

use crate::solver::Solver;

pub fn solve_day(day: u32) {
    let filename = format!("inputs/day{:02}.txt", day);

    let solution = match day {
        1 => day_01::Solution {},
        _ => panic!("No solution found for that day!"),
    };

    println!("*=============================*");
    println!("|     Advent of Code 2020     |");
    println!("*=============================*");
    println!();
    println!("Solutions for day {}:\n", day);

    solution.solve(String::from(filename));
}
