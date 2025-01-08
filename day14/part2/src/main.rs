#[path="../../../modules/readdata.rs"]
mod readdata;
mod robot;
mod painter;

use std::env;
use robot::Robot;
use painter::Painter;
use std::io;
use std::io::prelude::*;

fn main() {
  println!("Day 14 - Part 2");

  let mut input_file:String = "../test.txt".to_string();
  let mut cols = 11;
  let mut rows = 7;
  let mut step_size = 1;
  
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
    5 => {
      input_file = args[1].clone();
      cols = args[2].parse().unwrap();
      rows = args[3].parse().unwrap();
      step_size = args[4].parse().unwrap();
    },
    _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut robots: Vec<Robot> = Vec::new();
  for line in data {
    let robot = Robot::new_from_string(&line, cols, rows);
    robots.push(robot);
  }

  

  let mut painter = Painter::new((cols * 2).try_into().unwrap(), (rows * 2).try_into().unwrap());
  for robot in &mut robots {
    for time in 0..31 {
      robot.patrol();
    }
  }
  loop {
    let mut x_checks:Vec<usize> = vec![0; cols as usize];
    let mut y_checks:Vec<usize> = vec![0; rows as usize];

    for robot in &mut robots {
      for time in 0..step_size {
        robot.patrol();
        if time == step_size - 1 {
          x_checks[robot.get_x() as usize] += 1;
          y_checks[robot.get_y() as usize] += 1;
        }
      }
    }

    let mut should_pause = false
    for x in 0..cols {
      let x_size = x_checks[x as usize];
      let x_break_size = rows/3;
      if x_size > x_break_size {
        should_pause = true;
        break;
      }
    }
    for y in 0..rows {
      if y_checks[y as usize] > cols/3 {
        should_pause = true;
        break;
      }
    }
    if should_pause {
      println!("Robot reports: {0}", robots[0].get_move_count());
      painter.draw("output.png", &robots);
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      if input.trim() == "exit" {
        break;
      }
    }
  }  
}
