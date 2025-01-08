use regex::Regex;
use crate::guard::Guard;
use crate::guard::Direction;

#[derive(Clone)]
pub struct Grid {
  griddata: Vec<(usize, usize)>,
  pub rows: usize,
  pub cols: usize,
  pub guard: Option<Guard>,
}

impl Grid {
  pub fn new() -> Grid {
    Grid {
      griddata: Vec::new(),
      rows: 0,
      cols: 0,
      guard: None,
    }
  }
  
  pub fn obstacle_count(&self) -> usize {
    self.griddata.len()
  }

  pub fn add_row(&mut self, row:String) {
    let row_vector:Vec<char> = row.chars().collect();
    if row_vector.len() > self.cols {
      self.cols = row_vector.len();
    }
    
    let reg = Regex::new(r"#").unwrap();
    for m in reg.find_iter(&row).map(|f| f.start()) {
      self.griddata.push((m, self.rows));
    }

    let reg_guard = Regex::new(r"\^").unwrap();
    for m in reg_guard.find_iter(&row).map(|f| f.start()) {
      self.guard = Some(Guard { 
        x: m, 
        y: self.rows,
        direction: Direction::North,
      });
    }
    self.rows += 1;
  }

  pub fn add_obstacle(&mut self, x:usize, y:usize) {
    self.griddata.push((x, y));
  }

  pub fn move_guard(&mut self) -> bool {
    let guard = self.guard.expect("Guard doesn't exist");
    let mut next_x:usize = guard.x;
    let mut next_y:usize = guard.y;
    

    match guard.direction {
      Direction::North => { if next_y == 0 { return false; } next_y -= 1; }
      Direction::East => { next_x += 1; }
      Direction::South => { next_y += 1; }
      Direction::West => { if next_x == 0 { return false; } next_x -= 1; }
    }

    if self.griddata.contains(&(next_x, next_y)) {
      self.guard.as_mut().expect("No guard").turn();
      return self.move_guard();
    }

    // out of bounds
    if next_x >= self.cols || next_y >= self.rows {
      return false;
    }
    
    self.guard.as_mut().expect("No guard").set_position(next_x, next_y);

    return true;
  }

  pub fn has_loop(&mut self) -> bool {
    let mut moved = true;
    let mut move_count = 0;
    while moved && move_count < 10000 {
      moved = self.move_guard();
      move_count += 1;
    }
    return moved;
  }

  pub fn print_grid(&self) {
    let guard = self.guard.expect("Guard doesn't exist");
    for test_y in 0..self.rows {
      for test_x in 0..self.cols {
        if guard.x == test_x && guard.y == test_y {
          print!("^");
        } else if self.griddata.contains(&(test_x, test_y)) {
          print!("#");
        } else {
          print!(".");
        }
      }
      println!("");
    }
  }
}
