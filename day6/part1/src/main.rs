#[path="../../../modules/readdata.rs"]
mod readdata;
mod grid;
mod guard;

use std::env;
use crate::grid::Grid;
use std::collections::HashSet;

fn main() {
  println!("Day 6 - Part 1");

  let mut input_file:String = "../test.txt".to_string();

  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
    _ => {}
  }

  let data = readdata::read_input(&input_file);
  
  let mut board = Grid::new();

  for line_num in 0..data.len() {
    let line = data[line_num].clone();
    let _ = &board.add_row(line);
  }

  // println!("Obstacles: {0}", board.obstacle_len());
  let mut guard = board.guard.unwrap();
  // println!("Guard: {0}, {1}", guard.x, guard.y);
  
  let mut moved = true;
  let mut moves:HashSet<String> = [format!("{0}-{1}", guard.x, guard.y)].into();
  while moved {
    moved = board.move_guard();
    guard = board.guard.unwrap();
    // println!("Guard: {0}, {1}", guard.x, guard.y);
    moves.insert(format!("{0}-{1}", guard.x, guard.y));
  }

  println!("Squares: {0}", moves.len());
}
