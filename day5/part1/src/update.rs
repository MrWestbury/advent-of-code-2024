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
}