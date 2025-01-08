
fn main() {
  let x_moves_a = 94;
  let y_moves_a = 34;
  let x_moves_b = 22;
  let y_moves_b = 67;
  let x_target = 8400;
  let y_target = 5400;

  // (unknown_a * x_moves_a) + (unknown_b * y_moves_a) = x_target
  // (unknown_a * x_moves_b) + (unknown_b * y_moves_b) = y_target

  // 94a + 34b = 8400
  // 22a + 67b = 5400
  //[ 94 34 | 8400 ]
  //[ 22 67 | 5400 ]->

  let det_a = x_moves_a * y_moves_b - x_moves_b * y_moves_a;
  if det_a == 0 {
    println!("Determinant is zero");
    return;
  }

  let det_a_a = x_target * y_moves_b - x_moves_b * y_target;
  let det_a_b = x_moves_a * y_target - x_target * y_moves_a;

  let unknown_a = det_a_a / det_a;
  let unknown_b = det_a_b / det_a;
  
  println!("Unknown A: {0}\nUnknown B: {1}", unknown_a, unknown_b);
}
