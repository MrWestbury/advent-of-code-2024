use std::collections::HashMap;
use crate::regional_point::RegionalPoint;

pub struct Region {
  pub id:usize,
  pub letter:char,
  pub points:HashMap<(usize,usize), RegionalPoint>,
  pub fences:i64,
}

impl Region {
  pub fn new(id:usize, letter:char) -> Self {
    Region {
      id: id,
      letter: letter,
      points: HashMap::new(),
      fences: 0,
    }
  }

  pub fn add_point(&mut self, p:RegionalPoint) {
    self.points.insert((p.x, p.y), p);
  }

  pub fn add_fence(&mut self) {
    self.fences += 1;
  }

  pub fn get_fences(&self) -> i64 {
    return self.fences;
  }

  pub fn get_id(&self) -> usize {
    return self.id;
  }

  pub fn get_letter(&self) -> char {
    return self.letter;
  }

  pub fn get_points(&self) -> &HashMap<(usize, usize), RegionalPoint> {
    return &self.points;
  }

  pub fn get_area(&self) -> usize {
    return self.points.len();
  }

  pub fn get_score(&self) -> i64 {
    return self.find_corners() * self.get_area() as i64;
  }

  pub fn find_corners(&self) -> i64 {
    let mut corner_count:i64 = 0;
    for ((x, y), point) in self.points.iter() {
      let mut left_same = true;
      let mut right_same = true;
      let mut top_same = true;
      let mut bottom_same = true;
    
      if *x == 0 || !self.points.contains_key(&(*x-1, *y)) {
        left_same = false;
      }

      if *y == 0 || !self.points.contains_key(&(*x, *y-1)) {
        top_same = false;
      }

      if !self.points.contains_key(&(*x+1, *y)) {
        right_same = false;
      }

      if !self.points.contains_key(&(*x, *y+1)) {
        bottom_same = false;
      }

      // Outer Top Left
      if !left_same && !top_same {
        corner_count += 1;
      }

      // Outer Top Right
      if !right_same && !top_same {
        corner_count += 1;
      }

      // Outer Bottom Left
      if !left_same && !bottom_same {
        corner_count += 1;
      }

      // Outer Bottom Right
      if !right_same && !bottom_same {
        corner_count += 1;
      }

      // Inner Top Left
      if left_same && top_same && !self.points.contains_key(&(*x-1, *y-1)) {
        corner_count += 1;
      }

      // Inner Top Right
      if right_same && top_same && !self.points.contains_key(&(*x+1, *y-1)) {
        corner_count += 1;
      }

      // Inner Bottom Left
      if left_same && bottom_same && !self.points.contains_key(&(*x-1, *y+1)) {
        corner_count += 1;
      }

      // Inner Bottom Right
      if right_same && bottom_same && !self.points.contains_key(&(*x+1, *y+1)) {
        corner_count += 1;
      }
    }
    return corner_count;
  }

}