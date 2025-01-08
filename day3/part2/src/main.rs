#[path="../../../modules/readdata.rs"]
mod readdata;
mod record;

use record::Record;
use regex::Regex;

fn main() {
  println!("Day 3 - Part 2");

  // let args: Vec<String> = env::args().collect();
  let data = readdata::read_input("../input.txt");
  let mut total:i32 = 0;
  let do_dont_reg = Regex::new(r"(do\(\)|don't\(\))").unwrap();
  let mul_reg = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
  let mut ignore = false;
  for line_num in 0..data.len() {
    let line = data[line_num].clone();
    let mut records = find(&do_dont_reg, &line);
    let mut results = find(&mul_reg, &line);
    records.append(&mut results);
    records.sort();
    for item in records {
      if item.text == "do()" {
        ignore = false;
      } else if item.text == "don't()" {
        ignore = true;
      } else if !ignore {
        let answer = calc_mul(&item);
        println!("Match: {2} {0} -> {1}", item.text, answer, item.position);
        total += answer;
      }
    }
  }
  
  println!("Answer: {0}", total);
}

fn find(reg:&Regex, data:&String) -> Vec<Record> {
  let mut results:Vec<Record> = Vec::new();
  for capture in reg.captures_iter(data).map(|c| c.get(0).unwrap()) {
    let substr = capture.as_str();
    let idx = capture.start();
    let record = Record {
      position: idx,
      text: substr.to_string(),
    };
    results.push(record);
  }
  return results;
}

fn calc_mul(data:&Record) -> i32 {
  let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
  let Some(captures) = re.captures(&data.text) else { return 0; };
  assert_eq!(3, captures.len());
  let num1:i32 = captures[1].parse().unwrap();
  let num2:i32 = captures[2].parse().unwrap();
  let result = num1 * num2;
  return result;
}