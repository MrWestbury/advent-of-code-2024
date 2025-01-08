use crate::warehousebox::WarehouseBox;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Grid {
  pub rows: i32,
  pub cols: i32,
  walls: HashSet<(i32, i32)>,
  boxes: HashMap<(i32, i32), WarehouseBox>,
  robot: (i32, i32),
}

impl Grid {
  pub fn new() -> Self {
    Grid {
      rows: 0,
      cols: 0,
      walls: HashSet::new(),
      boxes: HashMap::new(),
      robot: (0, 0),
    }
  }

  pub fn add_row(&mut self, row:String) {
    let chars:Vec<char> = row.chars().collect();
    let char_len = chars.len() as i32;
    if char_len > self.cols {
      self.cols = char_len;
    }

    for idx in 0..chars.len() {
      match chars[idx] {
        '#' => {
          self.walls.insert((idx as i32, self.rows));
        },
        'O' => {
          self.boxes.insert((idx as i32, self.rows), WarehouseBox::new(idx as i32, self.rows));
        },
        '@' => {
          self.robot = (idx as i32, self.rows);
        },
        _ => {}
      }
    }
    self.rows += 1;
  }

  pub fn moveme(&mut self, direction:char) {
    let (x, y) = self.robot;
    let (dx, dy) = match direction {
      '^' => (0, -1),
      'v' => (0, 1),
      '<' => (-1, 0),
      '>' => (1, 0),
      _ => (0, 0),
    };

    let new_x = x + dx;
    let new_y = y + dy;

    if self.walls.contains(&(new_x, new_y)) {
      return;
    }

    if self.boxes.contains_key(&(new_x, new_y)) {
      let success = self.move_box(new_x, new_y, dx, dy);
      if !success {
        return;
      }
    }

    self.robot = (new_x, new_y);
  }

  fn move_box(&mut self, x:i32, y:i32, dx:i32, dy:i32) -> bool {
    let new_x = x as i32 + dx;
    let new_y = y as i32 + dy;

    if self.walls.contains(&(new_x, new_y)) {
      return false;
    }

    if self.boxes.contains_key(&(new_x, new_y)) {
      let cascade = self.move_box(new_x, new_y, dx, dy);
      if !cascade {
        return false;
      }

    }

    let wbox = self.boxes.get_mut(&(x, y)).unwrap();
    wbox.moveme(dx, dy);
    let newbox = wbox.clone();
    self.boxes.remove(&(x, y));
    self.boxes.insert((new_x, new_y), newbox);
    return true;
  }

  pub fn print(&self) {
    for r in 0..self.rows {
      for c in 0..self.cols {
        if self.walls.contains(&(c, r)) {
          print!("#");
        } else if self.boxes.contains_key(&(c, r)) {
          print!("O");
        } else if self.robot == (c, r) {
          print!("@");
        } else {
          print!(".");
        }
      }
      println!();
    }
  }

  pub fn get_score(&self) -> i32 {
    let mut score = 0;
    for (_k, v) in &self.boxes {
      println!("Test Box: {0} {1} = {2}", v.get_x(), v.get_y(), v.get_gps());
      score += v.get_gps();
    }    
    return score;
  }
}