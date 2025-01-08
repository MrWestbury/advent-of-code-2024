use crate::warehousebox::WarehouseBox;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct Grid {
  pub rows: i32,
  pub cols: i32,
  walls: HashSet<(i32, i32)>,
  boxes: HashMap<i32, WarehouseBox>,
  robot: (i32, i32),
}

impl Grid {
  pub fn new() -> Self {
    Grid {
      rows: 0,
      cols: 0,
      walls: HashSet::new(),
      boxes: HashSet::new(),
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
          let wbox = WarehouseBox::new(idx as i32, self.rows);
          self.boxes.insert(self.boxes.len() as i32, wbox);
        },
        '@' => {
          self.robot = (idx as i32, self.rows);
        },
        _ => {}
      }
    }
    self.rows += 1;
  }


  // moveme
  //
  // This function will move the robot in the direction specified.
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

    let mut tmpbox = self.get_box_id_at(new_x, new_y);
    if tmpbox.is_some() {
      let success = self.move_box(tmpbox, dx, dy);
      if !success {
        return;
      }
    }

    self.robot = (new_x, new_y);
  }

  // move_box
  //
  // This function is recursive
  // It will move the box in the direction specified
  // If the box is blocked by a wall, it will return false
  fn move_box(&mut self, tbox_id:i32, dx:i32, dy:i32) -> bool {
    let mut tbox = self.boxes.get_mut(&tbox_id).unwrap();
    let new_x = tbox.get_x() as i32 + dx;
    let new_x2 = tbox.get_x2() as i32 + dx;
    let new_y = tbox.get_y() as i32 + dy;

    if self.walls.contains(&(new_x, new_y)) {
      return false;
    }

    let mut can_move = true;

    let left_box_id = self.get_box_id_at(new_x, new_y);
    if left_box_id.is_some() {
      let cascade = self.move_box(left_box_id.unwrap(), dx, dy);
      if !cascade {
        can_move = false;
      }
    }

    let right_box_id = self.get_box_id_at(new_x2, new_y);
    if right_box_id.is_some() {
      let cascade = self.move_box(right_box_id.unwrap(), dx, dy);
      if !cascade {
        can_move = false;
      }
    }

    if can_move {
      tbox.moveme(dx, dy);
    }
    return can_move;
  }

  fn get_box_id_at(&self, x:i32, y:i32) -> Option<i32> {
    for id, b in self.boxes.iter() {
      if b.is_at_position(x, y) {
        return Some(id);
      }
    }
    return None;
  }

  fn is_box_at(&self, x:i32, y:i32) -> bool {
    for _id, b in self.boxes.iter() {
      if b.is_at_position(x, y) {
        return true;
      }
    }
    return false;
  }

  pub fn print(&self) {
    for r in 0..self.rows {
      for c in 0..self.cols {
        if self.walls.contains(&(c, r)) {
          print!("##");
        } else if self.is_box_at(c, r) {
          print!("[]");
        } else if self.robot == (c, r) {
          print!("@.");
        } else {
          print!("..");
        }
      }
      println!();
    }
  }

  pub fn get_score(&self) -> i32 {
    let mut score = 0;
    for k, v in &self.boxes {
      println!("Test Box: {0} {1} = {2}", v.get_x(), v.get_y(), v.get_gps());
      score += v.get_gps();
    }    
    return score;
  }
}