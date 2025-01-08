use std::collections::HashMap;
use crate::regional_point::RegionalPoint;
use crate::point::Point;
use crate::region::Region;
use std::hash::Hash;
use std::collections::HashSet;

pub struct Grid {
  pub rows: usize,
  pub cols: usize,
  topology: HashMap<(usize, usize), RegionalPoint>,
  pub regions: HashMap<usize, Region>
}

impl Grid {
  pub fn new() -> Self {
    Grid {
      rows: 0,
      cols: 0,
      topology: HashMap::new(),
      regions: HashMap::new(),
    }
  }

  pub fn add_row(&mut self, row:String) {
    let chars:Vec<char> = row.chars().collect();
    if chars.len() > self.cols {
      self.cols = chars.len();
    }

    for idx in 0..chars.len() {
      let new_point:RegionalPoint = RegionalPoint::new_with_char(idx, self.rows, chars[idx]);
      self.topology.insert((idx, self.rows), new_point);
    }
    self.rows += 1;
  }

  pub fn get_point(&mut self, x:usize, y:usize) -> Option<RegionalPoint> {
    return self.topology.get(&(x, y)).copied();
  }

  pub fn populate_regions(&mut self) {
    for row in 0..self.rows {
      for col in 0..self.cols {
        let target = self.topology.get(&(col, row)).copied().unwrap();
        let target_region_id = target.get_region_id();
        if target_region_id == usize::MAX {
          self.flood_fill(&target);
        }
      }
    }
  }

  pub fn flood_fill(&mut self, pnt:&RegionalPoint) -> usize {
    let mut region_stack:Vec<Point> = Vec::new();
    region_stack.push(pnt.as_point());
    let region_id = self.regions.len();
    if pnt.get_region_id() != usize::MAX {
      return pnt.get_region_id();
    }

    loop {
      if region_stack.len() == 0 {
        break;
      }
      
      dedup(&mut region_stack);

      let current_point_coords:Point = region_stack.pop().expect("Region stack empty but passed check. Strange");
      let current_x = current_point_coords.x;
      let current_y = current_point_coords.y;
      let current_point:&mut RegionalPoint = self.topology.get_mut(&(current_x, current_y)).unwrap();
      current_point.set_region_id(region_id);
      let entry = self.regions.entry(region_id).or_insert(Region::new(region_id, pnt.region));
      entry.add_point(*current_point);

      if current_y > 0 {
        let above = self.topology.get(&(current_x, current_y - 1));
        let above = above.unwrap();
        if above.is_region(pnt.region) {
          if above.get_region_id() == usize::MAX {
            region_stack.push(above.as_point());
          }
        } else {
          entry.add_fence();
        }
      } else {
        entry.add_fence();
      }

      if current_x > 0 {
        let left = self.topology.get(&(current_x - 1, current_y));
        let left = left.unwrap();
        if left.is_region(pnt.region) {
          if left.get_region_id() == usize::MAX {
            region_stack.push(left.as_point());
          }
        } else {
          entry.add_fence();
        }
      } else {
        entry.add_fence();
      }

      if current_y < self.rows - 1 {
        let below = self.topology.get(&(current_x, current_y + 1));
        let below = below.unwrap();
        if below.is_region(pnt.region) {
          if below.get_region_id() == usize::MAX {
            region_stack.push(below.as_point());
          }
        } else {
          entry.add_fence();
        }
      } else {
        entry.add_fence();
      }
      
      if current_x < self.cols - 1 {
        let right = self.topology.get(&(current_x + 1, current_y));
        let right = right.unwrap();
        if right.is_region(pnt.region) {
          if right.get_region_id() == usize::MAX {
            region_stack.push(right.as_point());
          }
        } else {
          entry.add_fence();
        }
      } else {
        entry.add_fence();
      }
    }
    return region_id;
  }

  pub fn print(&self) {
    for row in 0..self.rows {
      for col in 0..self.cols {
        let pnt = self.topology.get(&(col, row)).unwrap();
        print!("{}", pnt.get_region_id());
      }
      println!();
    }

    for row in 0..self.rows {
      for col in 0..self.cols {
        let pnt = self.topology.get(&(col, row)).unwrap();
        print!("{}", pnt.region);
      }
      println!();
    }
  }
}

fn dedup<T: Eq + Hash + Copy>(v: &mut Vec<T>) { // note the Copy constraint
  let mut uniques = HashSet::new();
  v.retain(|e| uniques.insert(*e));
}