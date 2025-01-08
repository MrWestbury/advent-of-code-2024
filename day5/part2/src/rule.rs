use std::fmt;

pub struct Rule {
  number:i8,
  lower:Vec<i8>,
  higher:Vec<i8>,
}

impl Rule {
  pub fn new(num:i8) -> Self {
    Self {
      number: num,
      lower: Vec::new(),
      higher: Vec::new()
    }
  }

  pub fn add_lower(&mut self, num:i8) {
    let _ = &self.lower.push(num);
  }

  pub fn add_higher(&mut self, num:i8) {
    let _ = &self.higher.push(num);
  }

  pub fn in_higher(&self, check_num:i8) -> bool {
    return self.higher.contains(&check_num);
  }
}

impl fmt::Display for Rule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{0}", self.number)
  }
}