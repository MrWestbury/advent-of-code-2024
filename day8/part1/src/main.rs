#[path="../../../modules/readdata.rs"]
mod readdata;
mod grid;
#[path="../../../modules/point.rs"]
mod point;

use std::env;
use crate::grid::Grid;
use crate::point::Vector;
use crate::point::Point;

fn main() {
  println!("Day 8 - Part 1");

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

  let mut antinode_list:Vec<Point> = Vec::new();
  for frequency in board.list_frequency_chars() {
    let maps = board.list_coords(frequency);
    for i in maps {
      for k in maps {
        if i == k {
          continue;
        }

        let delta:Vector = *i - *k;
        let sum_option:Option<Point> = *i + delta;
        if sum_option != None {
          let sum = sum_option.unwrap();
          // println!("Coord: {6} {0} {1} + {2} {3} = {4} {5}", &i.x, &i.y, delta.delta_x, delta.delta_y, sum.x, sum.y, frequency);
          antinode_list.push(sum);
        } 
      }
    }
  }
  for node in antinode_list {
    board.add_antinode(node);
  }

  board.print_grid();
  println!("Antinodes: {0}", board.count_antinodes());
  Grid::t();
}
