#[path="../../../modules/readdata.rs"]
mod readdata;
mod calibration;

use std::env;
use calibration::Calibration;

fn main() {
  println!("Day 7 - Part 1");

  let mut input_file:String = "../test.txt".to_string();

  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
    _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut calibrations:Vec<Calibration> = Vec::new();
  let mut total_sum:i64 = 0;
  for line_num in 0..data.len() {
    let line = data[line_num].clone();
    let cal = Calibration::from_line(line);
    calibrations.push(cal.clone());
    let win = cal.can_calc();
    println!("Calibration: {0} {1}", cal, win);
    if win {
      total_sum += cal.target();
    }
  }
  println!("Total: {0}", total_sum);
  
}