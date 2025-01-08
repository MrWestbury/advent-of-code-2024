use regex::Regex;

pub struct Point {
  pub x: usize,
  pub y: usize,
}

pub struct Grid {
  griddata: Vec<Vec<char>>,
  pub rows: usize,
  pub cols: usize,
  x_idx: Vec<Point>,
}

impl Grid {
  pub fn new() -> Grid {
    Grid {
      griddata: Vec::new(),
      rows: 0,
      cols: 0,
      x_idx: Vec::new(),
    }
  }
  
  pub fn add_row(&mut self, row:String) {
    let row_vector:Vec<char> = row.chars().collect();
    if row_vector.len() > self.cols {
      self.cols = row_vector.len();
    }
    self.griddata.push(row_vector);
    let reg = Regex::new(r"A").unwrap();
    for m in reg.find_iter(&row).map(|f| f.start()) {
      self.x_idx.push(Point { x: m, y: self.rows });
    }
    self.rows = self.griddata.len();
  }

  pub fn get(&self, x: usize, y: usize) -> Option<char> {
    if x >= self.cols.try_into().unwrap() {
      return None;
    }
    if y >= self.rows.try_into().unwrap() {
      return None;
    }

    return Some(self.griddata[y][x]);
  }

  pub fn get_as(&self) -> &Vec<Point> {
    return &self.x_idx
  }
}
