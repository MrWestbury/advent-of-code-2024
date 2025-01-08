use std::fmt;

#[derive(Clone)]
pub struct Update {
  pub nums:Vec<i8>
}

impl Update {
  pub fn new(data:&str) -> Self {
    let numbers = data.split(',').map(|d| d.parse().unwrap()).collect();

    Self {
      nums: numbers,
    }
  }

  pub fn get_median(&self) -> i8 {
    let idx:f32 = self.nums.len() as f32 / 2.0;
    self.nums[idx.floor() as usize]
  }

  pub fn set(&mut self, idx:usize, new_value:i8) {
    self.nums[idx] = new_value;
  }
}

impl fmt::Display for Update {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{0}", &self.nums.clone().into_iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" "))
  }
}