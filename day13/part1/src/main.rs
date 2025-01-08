#[path="../../../modules/readdata.rs"]
mod readdata;
mod action;
mod claw_machine;

use std::env;
use regex::Regex;
use crate::claw_machine::ClawMachine;

// 34123 - Too high
// 33894 - Too high

fn main() {
  println!("Day 13 - Part 1");

  let mut input_file:String = "../test.txt".to_string();

  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);
  let all_input:String = data.join("\n");
  let reg = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();

  let mut results:Vec<ClawMachine> = Vec::new();
  for (_, [x_moves_a, y_moves_a, x_moves_b, y_moves_b, target_x, target_y]) in reg.captures_iter(&all_input).map(|c| c.extract()) {
    let x_moves_a:i64 = x_moves_a.parse().unwrap();
    let y_moves_a:i64 = y_moves_a.parse().unwrap();
    let x_moves_b:i64 = x_moves_b.parse().unwrap();
    let y_moves_b:i64 = y_moves_b.parse().unwrap();
    let target_x:i64 = target_x.parse().unwrap();
    let target_y:i64 = target_y.parse().unwrap();
    results.push(ClawMachine::new(x_moves_a, y_moves_a, x_moves_b, y_moves_b, target_x, target_y));
  }

  let mut total_cost:i64 = 0;
  for machine in results {
    match machine.solve() {
      Some(action) => {
        let cost = action.get_cost();
        println!("Cost: {}", cost);
        total_cost += cost;
      },
      None => {
        println!("No solution found");
      }
    }
  }
  println!("Total cost: {}", total_cost);
}
