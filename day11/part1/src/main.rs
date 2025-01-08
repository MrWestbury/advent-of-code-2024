#[path="../../../modules/readdata.rs"]
mod readdata;
mod node;

use std::env;
use crate::node::Node;

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

  let mut top_level:Vec<Node> = data[0].split(" ").map(|m| Node::new(m.parse().unwrap())).collect();

  for top_node in &mut top_level {
    for _idx in 0..25 {
      top_node.process(&cache);
    }
  }

  let mut total:usize = 0;
  let mut values:Vec<i64> = Vec::new();
  for top_node in &top_level {
    total += top_node.node_count();
    values.append(&mut top_node.values());
  }

  println!("Score: {0}", total);
}
