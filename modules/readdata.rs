use std::fs::read_to_string;

pub fn read_input(filename: &str) -> Vec<String> {
  read_to_string(filename)
      .unwrap()
      .lines()
      .map(String::from)
      .collect()
}