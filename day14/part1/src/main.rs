#[path="../../../modules/readdata.rs"]
mod readdata;
mod robot;

use std::env;
use robot::Robot;

// 221835159 - too high [10, 141, 113, 119, 117]
// 222674400 - too high [11, 130, 120, 122, 117]
fn main() {
  println!("Day 14 - Part 1");

  let mut input_file:String = "../test.txt".to_string();
  let mut cols = 11;
  let mut rows = 7;
  
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
    4 => {
      input_file = args[1].clone();
      cols = args[2].parse().unwrap();
      rows = args[3].parse().unwrap();
    },
    _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut quads = vec![0,0,0,0,0];
  for line in data {
    let mut robot = Robot::new_from_string(&line, cols, rows);
    for _step in 0..100 {
      robot.patrol();
    }
    println!("Robot: {0} {1:?}", robot.get_move_count(), robot.get_position());
    let quad = robot.quadrant();
    quads[quad] += 1;
  }
  println!("mid_x: {0} mid_y: {1}", (cols as isize - 1) / 2, (rows as isize - 1) / 2);
  println!("Quadrants: {:?}", quads);
  println!("Score: {}", quads[1] * quads[2] * quads[3] * quads[4]);
}
