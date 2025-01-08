#[path="../../../modules/readdata.rs"]
mod readdata;

use std::env;
use std::collections::HashMap;

fn main() {
  println!("Day 11 - Part 1");

  let mut input_file:String = "../test.txt".to_string();

  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut top_level = data[0].split(' ').collect::<Vec<&str>>().into_iter().map(|m| (m.parse().unwrap(), 1)).collect::<HashMap<i64, usize>>();

  for round in 0..75 {
    let mut next_round:HashMap<i64, usize> = HashMap::new();
    for (key, value) in top_level {
      let str_value = format!("{0}", key);
      if key == 0 {
        let entry = next_round.entry(1).or_insert(0);
        *entry += value;
      } else if str_value.len() % 2 == 0 {
        let left_value = str_value[..str_value.len()/2].parse().unwrap();
        let left_entry = next_round.entry(left_value).or_insert(0);
        *left_entry += value;

        let right_value = str_value[str_value.len()/2..].parse().unwrap();
        let right_entry = next_round.entry(right_value).or_insert(0);
        *right_entry += value;
      } else {
        let rest_value = key * 2024;
        let entry = next_round.entry(rest_value).or_insert(0);
        *entry += value;
      }
      
    }
    println!("Score: {0} {1}", round, next_round.len());
    top_level = next_round;
  }
  let total:usize = top_level.values().into_iter().sum();
  println!("Total: {0}", total);
}
