use std::collections::HashMap;
use std::collections::HashSet;
use crate::point::Point;

pub struct Grid {
  pub rows: usize,
  pub cols: usize,
  antennas: HashMap<Point, char>,
  antenna_ids: HashMap<char, HashSet<Point>>,
  antinodes: HashSet<Point>,
}

impl Grid {
  pub fn new() -> Grid {
    Grid {
      rows: 0,
      cols: 0,
      antennas: HashMap::new(),
      antenna_ids: HashMap::new(),
      antinodes: HashSet::new(),
    }
  }

  pub fn add_row(&mut self, row:String) {
    let chars:Vec<char> = row.chars().collect();
    if chars.len() > self.cols {
      self.cols = chars.len();
    }

    for idx in 0..chars.len() {
      if chars[idx] != '.' {
        let new_point:Point = Point::new(idx, self.rows);
        self.antennas.insert(new_point, chars[idx]);
        if !self.antenna_ids.contains_key(&chars[idx]) {
          self.antenna_ids.insert(chars[idx], HashSet::new());
        }
        self.antenna_ids.entry(chars[idx]).and_modify(|h| { h.insert(new_point); });
      }
    }
    self.rows += 1;
  }

  pub fn add_antinode(&mut self, point:Point) {
    if point.x < self.cols && point.y < self.rows {
      self.antinodes.insert(point);
    }
  }

  pub fn list_coords(&self, idx:char) -> &HashSet<Point> {
    return self.antenna_ids.get(&idx).unwrap();
  }

  pub fn list_frequency_chars(&self) -> Vec<char> {
    self.antenna_ids.clone().into_keys().collect()
  }

  pub fn count_antinodes(&self) -> usize {
    self.antinodes.len()
  }

  pub fn print_grid(&self) {
    for test_y in 0..self.rows {
      for test_x in 0..self.cols {
        let test_point:Point = Point::new(test_x, test_y);
        if self.antinodes.contains(&test_point) {
          print!("#");
        } else if self.antennas.contains_key(&test_point) {
          let chr = self.antennas.get(&test_point).unwrap();
          print!("{0}", chr);
        } else {
          print!(".");
        }
      }
      println!("");
    }
  }

  pub fn t() {
    let mut test:HashSet<Point> = HashSet::new();
    test.insert(Point::new(2, 8));
    test.insert(Point::new(5, 4));
    test.insert(Point::new(2, 8));
    println!("Test len {0}", test.len());
  }
}