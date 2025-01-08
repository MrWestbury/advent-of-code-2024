#[derive(Eq, Hash, PartialEq, Clone)]
pub struct WarehouseBox {
  x: i32,
  x2: i32,
  y: i32,
}

impl WarehouseBox {
  pub fn new(x: i32, y: i32) -> Self {
    Self {
      x,
      x2: x + 1,
      y,
    }
  }

  pub fn get_gps(&self) -> i32 {
    return self.x + (self.y * 100)
  }

  pub fn moveme(&mut self, dx:i32, dy:i32) {
    self.x += dx;
    self.x2 += dx;
    self.y += dy;
  }

  pub fn is_at_position(&self, x:i32, y:i32) -> bool {
    return self.y == y && (self.x == x || self.x2 == x);
  }

  pub fn get_x(&self) -> i32 {
    return self.x
  }
  pub fn get_x2(&self) -> i32 {
    return self.x2
  }

  pub fn get_y(&self) -> i32 {
    return self.y
  }
}