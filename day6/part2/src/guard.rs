#[derive(Copy, Clone)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

#[derive(Copy, Clone)]
pub struct Guard {
  pub x: usize,
  pub y: usize,
  pub direction: Direction,
}

impl Guard {
  pub fn turn(&mut self) {
    match self.direction {
      Direction::North => { self.direction = Direction::East; }
      Direction::East => { self.direction = Direction::South; }
      Direction::South => { self.direction = Direction::West; }
      Direction::West => { self.direction = Direction::North; }
    }
  }

  pub fn set_position(&mut self, new_x:usize, new_y:usize) {
    self.x = new_x;
    self.y = new_y;
  }
}