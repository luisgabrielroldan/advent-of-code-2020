mod solutions;
mod solver;

use std::env;

fn main() {
    let day = parse_day_from_args();

    solutions::solve_day(day);
}

fn parse_day_from_args() -> u32 {
    env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap_or(1)
}
