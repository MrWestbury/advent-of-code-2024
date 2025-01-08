fn main() {
  let data:Vec<i32> = [11,6,16,20].to_vec();
  //data.reverse();
  let win = calcs(&data, 292);
  println!("Match: {0}", win);
}

fn calcs(d:&Vec<i32>, target:i32) -> bool {
  let mut possibles:Vec<i32> = [d[0]].to_vec();
  for i in 1..d.len() {
    let mut tmp_possibles:Vec<i32> = Vec::new();
    for _idx in 0..possibles.len() {
      let current = possibles.pop().unwrap();
      let multiply_answer = d[i] * current;
      if multiply_answer <= target {
        tmp_possibles.push(multiply_answer);
      }
      let add_answer = d[i] + current;
      if add_answer <= target {
        tmp_possibles.push(add_answer);
      }
    }
    possibles = tmp_possibles;
  }
  possibles.contains(&target.clone())
}