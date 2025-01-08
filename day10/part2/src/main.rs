#[path="../../../modules/readdata.rs"]
mod readdata;
#[path="../../../modules/point.rs"]
mod point;
mod grid;

use std::env;
use crate::grid::Grid;
use crate::point::Point;
// use std::collections::HashSet;

fn main() {
  println!("Day 10 - Part 2");

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

  // map.print();
  
  let mut total:usize = 0;
  for start_point in map.get_layer(0) {
    let start_score = find_route_count(&map, start_point);
    total += start_score;
  }
  println!("Answer: {0}", total);
  
}

fn find_route_count(map:&Grid, start:Point) -> usize {
  let mut current_layer:Vec<Point> = [start].to_vec();
  for layer_num in 0..9 {
    let mut next_layer:Vec<Point> = Vec::new();
    let next_layer_num = layer_num + 1;
    for layer_point in &current_layer {
      // What's above
      let top = map.get_top(*layer_point);
      if top != None {
        if map.verify_point(top.unwrap(), next_layer_num) {
          next_layer.push(top.unwrap());
        }
      }

      // Jokers to the right
      let right = map.get_right(*layer_point);
      if right != None {
        if map.verify_point(right.unwrap(), next_layer_num) {
          next_layer.push(right.unwrap());
        }
      }

      // I come from a land down under
      let bottom = map.get_bottom(*layer_point);
      if bottom != None {
        if map.verify_point(bottom.unwrap(), next_layer_num) {
          next_layer.push(bottom.unwrap());
        }
      }

      // Clowns to the left of me
      let left = map.get_left(*layer_point);
      if left != None {
        if map.verify_point(left.unwrap(), next_layer_num) {
          next_layer.push(left.unwrap());
        }
      }
    }
    // println!("{0}: {1}", layer_num, &current_layer.len());
    current_layer = next_layer;
  }
  // let score:HashSet<Point> = HashSet::from_iter(current_layer.iter().cloned());
  return current_layer.len();
}

fn print_layer(cols:usize, rows:usize, points:&Vec<Point>) {
  for y in 0..rows {
    for x in 0..cols {
      let pnt = Point::new(x, y);
      if points.contains(&pnt) {
        print!("*");
      } else {
        print!(".");
      }
    }
    println!("");
  }
}