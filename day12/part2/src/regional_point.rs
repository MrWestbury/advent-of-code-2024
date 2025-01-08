use crate::point::Point;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct RegionalPoint {
  pub x: usize,
  pub y: usize,
  pub region:char,
  region_id:usize,
}

impl RegionalPoint {
  pub fn new_with_char(x:usize, y:usize, c:char) -> Self {
    RegionalPoint {
      x: x,
      y: y,
      region: c,
      region_id: usize::MAX,
    }
  }

  pub fn set_region_id(&mut self, id:usize) {
    self.region_id = id;
  }

  pub fn get_region_id(&self) -> usize {
    return self.region_id;
  }

  pub fn is_region(&self, c:char) -> bool {
    return self.region == c;
  }

  pub fn as_point(&self) -> Point {
    Point::new(self.x, self.y)
  }
}