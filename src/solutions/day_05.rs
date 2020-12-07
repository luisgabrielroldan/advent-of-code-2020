use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use crate::solver::Solver;

pub struct Solution;

impl Solver for Solution {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Self::Input {
        BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
      // let passes: Vec<(usize, usize)> = Vec::new();
      let mut highest_id = 0;

      for pass in input {
        let (_, _, id) = decode_boarding_pass(pass);

        if id > highest_id {
          highest_id = id;
        }
      }

      highest_id
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {

      let mut seats: Vec<usize> = input
        .iter()
        .map(|line| {
          let (_, _, id) = decode_boarding_pass(line);

          id
        })
      .collect();

      seats.sort();

      let mut s = seats[0];

      for i in 0..seats.len() {
        if s != seats[i] {
          return s;
        }
        s += 1;
      }

      println!("{:?}", seats);

      panic!("Seat not found");
    }
}

fn decode_boarding_pass(pass: &String) -> (usize, usize, usize) {
  let row_seq = String::from(&pass[..7]);
  let column_seq = String::from(&pass[7..]);
  let row = decode_seq(&row_seq, 128);
  let column = decode_seq(&column_seq, 8);
  let seat_id = row * 8 + column;

  (row,column, seat_id)
}

fn decode_seq(seq: &String, total: usize) -> usize {
  let mut set: Vec<usize> = (0..total).collect();
  for c in seq.chars() {
    match c {
      'F' | 'L' => { set = set[..(set.len()/2)].to_vec(); }
      'B' | 'R' => { set = set[(set.len()/2)..].to_vec(); }
      _ => panic!("Unexpected boarding pass char {}", c)
    }
  }

  set[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_seq() {
      let value1 = decode_seq(&"FBFBBFF".to_string(), 128);
      let value2 = decode_seq(&"RLR".to_string(), 8);

      assert!(value1 == 44);
      assert!(value2 == 5);
    }

    #[test]
    fn test_decode_boarding_pass() {
      let (row, column) = decode_boarding_pass(&"FBFBBFFRLR".to_string());

      assert!(row == 44);
      assert!(column == 5);
    }

    #[test]
    fn test_first() {
    }

    #[test]
    fn test_second() {
    }
}
