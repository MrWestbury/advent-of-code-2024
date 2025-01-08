use std::ops;

#[derive(Copy,Clone)]
pub struct Vector {
  pub delta_x: i32,
  pub delta_y: i32,
}

impl Vector {
  pub fn new(dx:i32, dy:i32) -> Vector {
    Vector {
      delta_x: dx,
      delta_y: dy
    }
  }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct Point {
  pub x: usize,
  pub y: usize,
}

impl Point {
  pub fn new(x:usize, y:usize) -> Point {
    Point {
      x: x,
      y: y
    }
  }
}

impl ops::Add<Vector> for Point {
  type Output = Option<Point>;

  fn add(self, rhs:Vector) -> Option<Point> {
    let new_x = self.x as i32 + rhs.delta_x;
    let new_y = self.y as i32 + rhs.delta_y;
    if new_x < 0 || new_y < 0 {
      return None;
    }
    Some(Point {
      x: new_x as usize,
      y: new_y as usize
    })
  }
}

impl ops::Sub<Point> for Point {
  type Output = Vector;

  fn sub(self, rhs:Point) -> Vector {
    let dx = self.x as i32 - rhs.x as i32;
    let dy = self.y as i32 - rhs.y as i32;
    Vector::new(dx, dy)
  }
}