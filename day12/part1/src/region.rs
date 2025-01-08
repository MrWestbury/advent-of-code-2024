use std::collections::HashSet;
use crate::regional_point::RegionalPoint;

pub struct Region {
  pub id:usize,
  pub letter:char,
  pub points:HashSet<RegionalPoint>,
  pub fences:i64,
}

impl Region {
  pub fn new(id:usize, letter:char) -> Self {
    Region {
      id: id,
      letter: letter,
      points: HashSet::new(),
      fences: 0,
    }
  }

  pub fn add_point(&mut self, p:RegionalPoint) {
    self.points.insert(p);
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

  pub fn get_points(&self) -> &HashSet<RegionalPoint> {
    return &self.points;
  }

  pub fn get_area(&self) -> usize {
    return self.points.len();
  }

  pub fn get_score(&self) -> i64 {
    return self.fences * self.get_area() as i64;
  }
}