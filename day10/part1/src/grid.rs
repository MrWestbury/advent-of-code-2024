use std::collections::HashMap;
use crate::point::Point;

pub struct Grid {
  pub rows: usize,
  pub cols: usize,
  topology: HashMap<Point, usize>,
  reverse_topology: HashMap<usize, Vec<Point>>,
}

impl Grid {
  pub fn new() -> Self {
    let mut rev_top:HashMap<usize, Vec<Point>> = HashMap::new();
    for i in 0..10 {
      let idx:usize = i.try_into().unwrap();
      rev_top.insert(idx, Vec::new());
    }

    Grid {
      rows: 0,
      cols: 0,
      topology: HashMap::new(),
      reverse_topology: rev_top,
    }
  }

  pub fn add_row(&mut self, row:String) {
    let chars:Vec<char> = row.chars().collect();
    if chars.len() > self.cols {
      self.cols = chars.len();
    }

    for idx in 0..chars.len() {
      let new_point:Point = Point::new(idx, self.rows);
      let height_as_int:u32 = chars[idx].to_digit(10).unwrap();
      let height:usize = height_as_int.try_into().unwrap();
      self.topology.insert(new_point, height);
      self.reverse_topology.get_mut(&height).unwrap().push(new_point)
    }
    self.rows += 1;
  }

  pub fn print(&self) {
    for row in 0..self.rows {
      for col in 0..self.cols {
        let pnt = Point::new(col, row);
        let value = self.topology.get(&pnt).unwrap();
        print!("{0}", value);
      }
      println!("");
    }
  }

  pub fn get_height(&self, pnt:Point) -> Option<usize> {
    return self.topology.get(&pnt).copied();
  } 

  pub fn get_layer(&self, height:usize) -> Vec<Point> {
    return self.reverse_topology.get(&height).unwrap().clone();
  }

  pub fn get_top(&self, pnt:Point) -> Option<Point> {
    if pnt.y == 0 || pnt.x > self.cols {
      return None;
    }
    return Some(Point::new(pnt.x, pnt.y - 1));
  }

  pub fn get_right(&self, pnt:Point) -> Option<Point> {
    if pnt.x >= self.cols || pnt.y > self.rows {
      return None;
    }
    return Some(Point::new(pnt.x + 1, pnt.y));
  }

  pub fn get_bottom(&self, pnt:Point) -> Option<Point> {
    if pnt.x > self.cols || pnt.y >= self.rows {
      return None;
    }
    return Some(Point::new(pnt.x, pnt.y + 1));
  }

  pub fn get_left(&self, pnt:Point) -> Option<Point> {
    if pnt.x == 0 || pnt.y > self.rows {
      return None;
    }
    return Some(Point::new(pnt.x -1, pnt.y));
  }

  pub fn verify_point(&self, pnt:Point, target:usize) -> bool {
    let pnt_opt = self.get_height(pnt);
    match pnt_opt {
      Some(_usize) => {
        return pnt_opt.unwrap() == target;
      }
      _ => { return false; }
    }
  }
}