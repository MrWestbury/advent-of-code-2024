// use std:env;
use regex::Regex;

#[path="../../../modules/readdata.rs"]
mod readdata;

fn main() {
  println!("Day 3 - Part 1");

  // let args: Vec<String> = env::args().collect();

  let data = readdata::read_input("../input.txt");
  let mut total:i32 = 0;
  
  for line_num in 0..data.len() {
    let line = data[line_num].clone();
  
    let results = get_muls(&line);
  
    for item in results {
      let answer = calc_mul(&item.to_string());
      println!("Match: {0} -> {1}", item, answer);
      total += answer;
    }
  }
  
  println!("Answer: {0}", total);
}

fn get_muls(data:&String) -> Vec<String> {
  let reg = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
  let mut results:Vec<String> = Vec::new();
  for (_, [data_point]) in reg.captures_iter(data).map(|c| c.extract()) {
    results.push(data_point.to_string());
  }
  return results;
}

fn calc_mul(data:&String) -> i32 {
  let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
  let Some(captures) = re.captures(data) else { return 0; };
  assert_eq!(3, captures.len());
  let num1:i32 = captures[1].parse().unwrap();
  let num2:i32 = captures[2].parse().unwrap();
  let result = num1 * num2;
  return result;
}