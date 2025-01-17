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
      boxes: HashMap::new(),
      robot: (0, 0),
    }
  }

  pub fn add_row(&mut self, row:String) {
    let chars:Vec<char> = row.chars().collect();
    let char_len = chars.len() as i32 * 2;
    if char_len > self.cols {
      self.cols = char_len;
    }

    for idx in 0..chars.len() {
      match chars[idx] {
        '#' => {
          self.walls.insert((idx as i32 * 2, self.rows));
          self.walls.insert((idx as i32 * 2 + 1, self.rows));
        },
        'O' => {
          let wbox = WarehouseBox::new(idx as i32 * 2, self.rows);
          self.boxes.insert(self.boxes.len() as i32, wbox);
        },
        '@' => {
          self.robot = (idx as i32 * 2, self.rows);
        },
        _ => {}
      }
    }
    self.rows += 1;
  }


  // move_robot
  //
  // This function will move the robot in the direction specified.
  pub fn move_robot(&mut self, direction:char) {
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

    let mut moves:HashSet<(i32, i32, i32)> = HashSet::new();
    let mut check_stack:Vec<i32> = Vec::new();

    let test_box_id = self.get_box_id_at(new_x, new_y);
    if test_box_id.is_some() {
      check_stack.push(test_box_id.unwrap());
    }
    let mut can_move = true;
    loop {
      if check_stack.is_empty() {
        break;
      }

      let box_id = check_stack.pop().unwrap();
      moves.insert((box_id, dx, dy));
      let tmpbox = self.boxes.get(&box_id).unwrap();
      match direction {
        '^' | 'v' => {
          let next_y = tmpbox.get_y() as i32 + dy;
          
          // Left side
          let next_x1 = tmpbox.get_x() as i32 + dx;
          if self.walls.contains(&(next_x1, next_y)) {
            // println!("Wall x1 hit at {0} {1}", next_x1, next_y);
            can_move = false;
            check_stack.clear();
            break;
          }
          
          let left_next_box_id = self.get_box_id_at(next_x1, next_y);
          if left_next_box_id.is_some() {
            check_stack.push(left_next_box_id.unwrap());
          }


          // Right side
          let next_x2 = tmpbox.get_x2() as i32 + dx;
          if self.walls.contains(&(next_x2, next_y)) {
            // println!("Wall x2 hit at {0} {1}", next_x2, next_y);
            can_move = false;
            check_stack.clear();
            break;
          }
          let right_next_box_id = self.get_box_id_at(next_x2, next_y);
          if right_next_box_id.is_some() && (left_next_box_id.is_none() || right_next_box_id.unwrap() != left_next_box_id.unwrap()) {
            check_stack.push(right_next_box_id.unwrap());
          }
        },
        '>' => {
          let next_x2 = tmpbox.get_x2() as i32 + dx;
          let next_y = tmpbox.get_y() as i32 + dy;
          
          if self.walls.contains(&(next_x2, next_y)) {
            can_move = false;
            check_stack.clear();
            break;
          }

          let right_next_box_id = self.get_box_id_at(next_x2, next_y);
          if right_next_box_id.is_some() {
            check_stack.push(right_next_box_id.unwrap());
          }
        },
        '<' => {
          let next_x1 = tmpbox.get_x() as i32 + dx;
          let next_y = tmpbox.get_y() as i32 + dy;
          
          if self.walls.contains(&(next_x1, next_y)) {
            can_move = false;
            check_stack.clear();
            break;
          }

          let left_next_box_id = self.get_box_id_at(next_x1, next_y);
          if left_next_box_id.is_some() {
            check_stack.push(left_next_box_id.unwrap());
          }
        },
        _ => {}
      }
    }
    //println!("Moves: {0}", moves.len());
    if can_move {
      for m in moves.iter() {
        self.move_box(m.0, m.1, m.2);
      }

      self.robot = (new_x, new_y);
    }
  }

  // move_box
  //
  // This function is recursive
  // It will move the box in the direction specified
  // If the box is blocked by a wall, it will return false
  fn move_box(&mut self, tbox_id:i32, dx:i32, dy:i32) {
    let tbox = self.boxes.get_mut(&tbox_id).unwrap();
    tbox.moveme(dx, dy);
  }

  fn get_box_id_at(&self, x:i32, y:i32) -> Option<i32> {
    for (id, b) in self.boxes.iter() {
      if b.is_at_position(x, y) {
        return Some(*id);
      }
    }
    return None;
  }

  fn is_box_at(&self, x:i32, y:i32) -> bool {
    for (_id, b) in self.boxes.iter() {
      if b.is_at_position(x, y) {
        return true;
      }
    }
    return false;
  }

  pub fn print(&self) {
    let mut last_open = false;
    for r in 0..self.rows {
      for c in 0..self.cols {
        if self.walls.contains(&(c, r)) {
          print!("#");
        } else if self.is_box_at(c, r) {
          if last_open {
            print!("]");
            last_open = false;
          } else {
            print!("[");
            last_open = true;
          }
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