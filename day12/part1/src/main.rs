#[path="../../../modules/readdata.rs"]
mod readdata;
mod grid;
mod region;
mod regional_point;
#[path="../../../modules/point.rs"]
mod point;

use std::env;
use crate::grid::Grid;

fn main() {
  println!("Day 12 - Part 1");

  let mut input_file:String = "../test.txt".to_string();

  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut map = Grid::new();

  for line_num in 0..data.len() {
    let line = data[line_num].clone();
    map.add_row(line);
  }

  map.populate_regions();
  let mut score:i64 = 0;
  for reg in map.regions.values() {
    println!("Region {0} ({3}) has {1} points and {2} fences", reg.get_id(), reg.get_area(), reg.get_fences(), reg.get_letter());
    score += reg.get_score();
  }
  
  println!("Score: {0}", score);
}
