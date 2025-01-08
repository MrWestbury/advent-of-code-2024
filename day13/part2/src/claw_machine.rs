use crate::action::Action;

pub struct ClawMachine {
  button_a_x_moves: i64,
  button_a_y_moves: i64,
  button_b_x_moves: i64,
  button_b_y_moves: i64,
  x_target: i64,
  y_target: i64,
}

impl ClawMachine {
  pub fn new(x_moves_a: i64, y_moves_a: i64, x_moves_b: i64, y_moves_b: i64, x_target: i64, y_target: i64) -> Self {
    Self {
      button_a_x_moves: x_moves_a,
      button_a_y_moves: y_moves_a,
      button_b_x_moves: x_moves_b,
      button_b_y_moves: y_moves_b,
      x_target: x_target + 10000000000000,
      y_target: y_target + 10000000000000,
    }
  }
  pub fn get_name(&self) -> String {
    return format!("Machine: X{}Y{}", self.x_target, self.y_target);
  }

  pub fn solve(&self) -> Option<Action> {
    println!("Solving for: {}", self.get_name());
    let det_a:i64 = self.button_a_x_moves * self.button_b_y_moves - self.button_b_x_moves * self.button_a_y_moves;
    if det_a == 0 {
      return None;
    }

    let det_a_a:i64 = self.x_target * self.button_b_y_moves - self.button_b_x_moves * self.y_target;
    let det_a_b:i64 = self.button_a_x_moves * self.y_target - self.x_target * self.button_a_y_moves;

    if det_a_a % det_a != 0 || det_a_b % det_a != 0 {
      return None;
    }

    let unknown_a:f64 = det_a_a as f64 / det_a as f64;
    let unknown_b:f64 = det_a_b as f64 / det_a as f64;

    
    println!("Unknown A: {}, Unknown B: {}", unknown_a, unknown_b);
    return Some(Action::new(unknown_a.round() as i64, unknown_b.round() as i64));
  }
}