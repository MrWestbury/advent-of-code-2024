#[derive(Eq, Hash, PartialEq, Clone)]
pub struct WarehouseBox {
  x: i32,
  y: i32,
}

impl WarehouseBox {
  pub fn new(x: i32, y: i32) -> Self {
    Self {
      x,
      y,
    }
  }

  pub fn get_gps(&self) -> i32 {
    return self.x + (self.y * 100)
  }

  pub fn moveme(&mut self, dx:i32, dy:i32) {
    self.x += dx;
    self.y += dy;
  }

  pub fn get_position(&self) -> (i32, i32) {
    return (self.x, self.y)
  }

  pub fn get_x(&self) -> i32 {
    return self.x
  }

  pub fn get_y(&self) -> i32 {
    return self.y
  }
}