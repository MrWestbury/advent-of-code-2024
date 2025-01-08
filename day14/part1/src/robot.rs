use regex::Regex;

pub struct Robot {
  x: isize,
  y: isize,
  vec_x: isize,
  vec_y: isize,
  rows: usize,
  cols: usize,
  move_count: usize,
}

impl Robot {
  pub fn new(x: isize, y: isize, vec_x: isize, vec_y: isize, rows: usize, cols: usize) -> Self {
    Self {
      x: x,
      y: y,
      vec_x: vec_x,
      vec_y: vec_y,
      rows: rows,
      cols: cols,
      move_count: 0,
    }
  }

  pub fn new_from_string(input: &str, cols: usize, rows: usize) -> Robot {
    let mut x = 0;
    let mut y = 0;
    let mut vec_x = 0;
    let mut vec_y = 0;

    // p=0,4 v=3,-3
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    for cap in re.captures_iter(input) {
      x = cap[1].parse().unwrap();
      y = cap[2].parse().unwrap();
      vec_x = cap[3].parse().unwrap();
      vec_y = cap[4].parse().unwrap();
    }
    
    return Self::new(x, y, vec_x, vec_y, rows, cols);
  }

  pub fn get_position(&self) -> (isize, isize) {
    (self.x, self.y)
  }

  pub fn get_move_count(&self) -> usize {
    self.move_count
  }

  pub fn patrol(&mut self) {
    let new_x = (self.x + self.vec_x) % self.cols as isize;
    let new_y = (self.y + self.vec_y) % self.rows as isize;
                
    if new_x >= 0 {
      self.x = new_x;
    } else {
      self.x = self.cols as isize + new_x;
    }

    if new_y >= 0 {
      self.y = new_y;
    } else {
      self.y = self.rows as isize + new_y;
    }
    self.move_count += 1;
  }

  pub fn quadrant(&self) -> usize {
    let mid_x = (self.cols as isize - 1) / 2;
    let mid_y = (self.rows as isize - 1) / 2;
    if self.x == mid_x || self.y == mid_y {
      return 0;
    } else if self.x < mid_x && self.y < mid_y {
      return 1;
    } else if self.x > mid_x && self.y < mid_y {
      return 2;
    } else if self.x < mid_x && self.y > mid_y {
      return 3;
    } else {
      return 4;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_patrol() {
    let mut robot = Robot::new(0, 0, 1, 0, 3, 3);
    robot.patrol();
    assert_eq!(robot.get_position(), (1, 0));
    robot.patrol();
    assert_eq!(robot.get_position(), (2, 0));
    robot.patrol();
    assert_eq!(robot.get_position(), (0, 0));
  }

  #[test]
  fn test_patrol_negative() {
    let mut robot = Robot::new(0, 0, -1, 0, 3, 3);
    robot.patrol();
    assert_eq!(robot.get_position(), (2, 0));
    robot.patrol();
    assert_eq!(robot.get_position(), (1, 0));
    robot.patrol();
    assert_eq!(robot.get_position(), (0, 0));
  }

  #[test]
  fn test_patrol_negative_y() {
    let mut robot = Robot::new(0, 0, 0, -1, 3, 3);
    robot.patrol();
    assert_eq!(robot.get_position(), (0, 2));
    robot.patrol();
    assert_eq!(robot.get_position(), (0, 1));
    robot.patrol();
    assert_eq!(robot.get_position(), (0, 0));
  }

  #[test]
  fn test_patrol_negative_x_y() {
    let mut robot = Robot::new(0, 0, -1, -1, 3, 3);
    robot.patrol();
    assert_eq!(robot.get_position(), (2, 2));
    robot.patrol();
    assert_eq!(robot.get_position(), (1, 1));
    robot.patrol();
    assert_eq!(robot.get_position(), (0, 0));
  }

  #[test]
  fn test_quadrant_0_x() {
    let robot = Robot::new(1, 0, 1, 0, 3, 3);
    assert_eq!(robot.quadrant(), 0);
  }

  #[test]
  fn test_quadrant_0_y() {
    let robot = Robot::new(0, 1, 0, 1, 3, 3);
    assert_eq!(robot.quadrant(), 0);
  }

  #[test]
  fn test_quadrant_1() {
    let robot = Robot::new(0, 0, 1, 1, 3, 3);
    assert_eq!(robot.quadrant(), 1);
  }

  #[test]
  fn test_quadrant_2() {
    let robot = Robot::new(2, 0, -1, 1, 3, 3);
    assert_eq!(robot.quadrant(), 2);
  }

  #[test]
  fn test_quadrant_3() {
    let robot = Robot::new(0, 2, 1, -1, 3, 3);
    assert_eq!(robot.quadrant(), 3);
  }

  #[test]
  fn test_quadrant_4() {
    let robot = Robot::new(2, 2, -1, -1, 3, 3);
    assert_eq!(robot.quadrant(), 4);
  }

  #[test]
  fn test_steps() {
    let mut robot = Robot::new(2, 4, 2, -3, 7, 11);
    robot.patrol();
    assert_eq!(robot.get_position(), (4, 1));
    robot.patrol();
    assert_eq!(robot.get_position(), (6, 5));
    robot.patrol();
    assert_eq!(robot.get_position(), (8, 2));
    robot.patrol();
    assert_eq!(robot.get_position(), (10, 6));
    robot.patrol();
    assert_eq!(robot.get_position(), (1, 3));
  }
}