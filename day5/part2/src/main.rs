#[path="../../../modules/readdata.rs"]
mod readdata;
mod rule;
mod update;

use std::collections::HashMap;
use rule::Rule;
use update::Update;

fn main() {
  println!("Day 5 - Part 2");

  let data = readdata::read_input("../test.txt");

  let mut mode_rules = true;
  let mut rules:HashMap<i8, Rule> = HashMap::new();
  let mut updates:Vec<Update> = Vec::new();
  for line_num in 0..data.len() {
    let line = data[line_num].clone();

    if line == "" {
      mode_rules = false;
      continue;
    }

    if mode_rules {
      let items:Vec<&str> = line.split("|").collect();
      let lower:i8 = items[0].parse().unwrap();
      let higher:i8 = items[1].parse().unwrap();
      
      let rl = rules.entry(lower).or_insert(Rule::new(lower));
      rl.add_higher(higher);

      let rh = rules.entry(higher).or_insert(Rule::new(higher));
      rh.add_lower(lower);
    } else {
      let up = Update::new(&line);
      let _ = updates.push(up);
    }
  }

  let mut total:i32 = 0;
  for item in updates {
    let result = check_rule(&item, &rules);
    if result {
      continue;
    }
    let new_update = fix_updates(&item, &rules);

    let median = new_update.get_median() as i32;
    total += median;
  }
  println!("Total: {0}", total);
}

pub fn check_rule(update:&Update, rules:&HashMap<i8, Rule>) -> bool {
  for idx in 0..update.nums.len() - 1 {
    let num = update.nums[idx];
    let next_num = update.nums[idx+1];
    let r = rules.get(&num).unwrap();
    let result = r.in_higher(next_num);
    if !result {
      return false;
    }
  }
  return true;
}

fn fix_updates(update:&Update, rules:&HashMap<i8, Rule>) -> Update {
  let mut temp_update = update.clone();
  println!("Old - {0}", temp_update);
  let mut is_valid = false;

  while !is_valid {
    let mut change_made = false;
    for idx in 0..temp_update.nums.len() - 1 {
      let num = temp_update.nums[idx];
      let next_num = temp_update.nums[idx+1];
      let r = rules.get(&num).unwrap();
      let result = r.in_higher(next_num);
      if !result {
        change_made = true;
        temp_update.set(idx, next_num);
        temp_update.set(idx+1, num);
      }
    }

    if !change_made {
      is_valid = true;
    }
  }
  println!("New - {0}", temp_update);
  return temp_update;
}