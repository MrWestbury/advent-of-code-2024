#[path="../../../modules/readdata.rs"]
mod readdata;
mod grid;
mod guard;

use std::env;
use crate::grid::Grid;
use std::collections::HashSet;

fn main() {
  println!("Day 6 - Part 2");

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

  let mut tested = 0;
  let mut loop_variations = 0;
  for test_y in 0..board.rows {
    for test_x in 0..board.cols {
      let mut test_board = board.clone();
      
      test_board.add_obstacle(test_x, test_y);
      let has_loop = test_board.has_loop();
      let guard = test_board.guard.unwrap();
      
      tested += 1;
      if has_loop {
        loop_variations += 1;
      }
    }
  }

  println!("Loops: {0}/{1}", loop_variations, tested);

  // let mut test_board = board.clone();
  // test_board.add_obstacle(1 as usize, 1 as usize);
  // test_board.print_grid();
  // let result = test_board.has_loop();
  // println!("Loop: {0}", result);
}
