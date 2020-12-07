mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

use crate::solver::Solver;

pub fn solve_day(day: u32) {
    let fp = String::from(format!("inputs/day{:02}.txt", day));

    println!("*=============================*");
    println!("|     Advent of Code 2020     |");
    println!("*=============================*");
    println!();
    println!("Solutions for day {}:\n", day);

    match day {
        1 => day_01::Solution {}.solve(fp),
        2 => day_02::Solution {}.solve(fp),
        3 => day_03::Solution {}.solve(fp),
        4 => day_04::Solution {}.solve(fp),
        5 => day_05::Solution {}.solve(fp),
        _ => panic!("No solution found for that day!"),
    };
}
