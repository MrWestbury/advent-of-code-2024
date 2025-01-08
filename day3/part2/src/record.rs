use std::cmp::Ordering;

pub struct Record {
  pub position: usize,
  pub text: String,
}

impl Ord for Record {
  fn cmp(&self, other: &Self) -> Ordering {
    self.position.cmp(&other.position)
  }
}

impl PartialOrd for Record {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Record {
  fn eq(&self, other: &Self) -> bool {
    self.text == other.text && self.position == other.position
  }
}

impl Eq for Record {}