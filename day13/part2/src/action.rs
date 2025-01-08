pub struct Action {
  pub x_presses: i64,
  pub y_presses: i64,
}

impl Action {
  pub fn new(x_presses: i64, y_presses: i64) -> Self {
    Self {
      x_presses,
      y_presses,
    }
  }

  pub fn get_cost(&self) -> i64 {
    return (self.x_presses * 3) + self.y_presses
  }
}