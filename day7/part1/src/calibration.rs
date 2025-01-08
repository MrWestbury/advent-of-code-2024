use std::fmt;

#[derive(Clone)]
pub struct Calibration {
  nums: Vec<i64>,
  target: i64
}

impl Calibration {
  pub fn from_line(line:String) -> Self {
    let splits:Vec<&str> = line.split(':').collect();
    let mut nums:Vec<&str> = splits[1].split(' ').collect();
    nums.retain(|&n| n != "");
    let new_calibration = Calibration{
      nums: nums.into_iter().map(|f| f.parse().unwrap()).collect(),
      target: splits[0].parse().unwrap()
    };
    
    return new_calibration;
  }

  pub fn target(&self) -> i64 {
    return self.target.clone();
  }

  pub fn can_calc(&self) -> bool {
    let mut possibles:Vec<i64> = [self.nums[0]].to_vec();
    for i in 1..self.nums.len() {
      let mut tmp_possibles:Vec<i64> = Vec::new();
      for _idx in 0..possibles.len() {
        let current = possibles.pop().unwrap();
        let multiply_answer = self.nums[i] * current;
        if multiply_answer <= self.target {
          tmp_possibles.push(multiply_answer);
        }
        let add_answer = self.nums[i] + current;
        if add_answer <= self.target {
          tmp_possibles.push(add_answer);
        }
      }
      possibles = tmp_possibles;
    }
    possibles.contains(&self.target.clone())
  }
}

impl fmt::Display for Calibration {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{0:?} = {1}", self.nums, self.target)
  }
}