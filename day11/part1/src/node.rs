#[derive(Clone)]
pub struct Node {
  pub value:i64,
  pub left_node:Option<Box<Node>>,
  pub right_node:Option<Box<Node>>,
}

impl Node {
  pub fn new(node_value:i64) -> Self {
    Node {
      value: node_value,
      left_node: None,
      right_node: None,
    }
  }

  pub fn process(&mut self) {
    let str_value = format!("{0}", self.value);
    if self.value == -1 {
      self.left_node.as_mut().unwrap().process();
      self.right_node.as_mut().unwrap().process();
    } else if self.value == 0 {
      self.value = 1;
    } else if str_value.len() % 2 == 0 {
      let left_value = str_value[..str_value.len()/2].parse().unwrap();
      let right_value = str_value[str_value.len()/2..].parse().unwrap();
      self.left_node = Some(Box::new(Node::new(left_value)));
      self.right_node = Some(Box::new(Node::new(right_value)));
      self.value = -1;
    } else {
      self.value = self.value * 2024;
    }
  }

  pub fn node_count(&self) -> usize {
    if self.value != -1 {
      return 1;
    } else {
      let left_count = self.left_node.as_ref().unwrap().node_count();
      let right_count = self.right_node.as_ref().unwrap().node_count();
      return left_count + right_count;
    }
  }

  pub fn values(&self) -> Vec<i64> {
    if self.value != -1 {
      return [self.value].to_vec();
    } else {
      let mut left = self.left_node.as_ref().unwrap().values();
      let mut right = self.right_node.as_ref().unwrap().values();
      left.append(&mut right);
      return left;
    }
  }
}