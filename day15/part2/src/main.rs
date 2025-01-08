#[path="../../../modules/readdata.rs"]
mod readdata;
mod grid;
mod warehousebox;

use std::env;
use crate::grid::Grid;

#[derive(PartialEq)]
enum Mode {
  Board,
  Directions,
}

fn main() {
  println!("Day 15 - Part 2");

  let mut input_file:String = "../test.txt".to_string();

  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut grid = Grid::new();
  let mut instructions = String::new();

  let mut mode = Mode::Board;
  for line in data {
    if mode == Mode::Board {
      if line == "" {
        mode = Mode::Directions;
      } else {
        grid.add_row(line);
      }
    } else {
      instructions += &line;
    }
  }
  println!("Data Load complete");

  for c in instructions.chars() {
    grid.moveme(c);
  }

  println!("Score: {}", grid.get_score()); 
}
