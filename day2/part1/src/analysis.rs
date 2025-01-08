pub fn analyse(data:String) -> bool {
  // println!("{}", data);
  let iter = data.split_whitespace();
  let mut inrc:i32 = 0;
  let mut last_value:i32 = -1;
  let mut count = 0;
  for str_value in iter {
    count += 1;
    let num_value:i32 = str_value.parse().unwrap();
    if last_value == -1 {
      last_value = num_value;
      continue;
    }

    let diff = (num_value - last_value).abs();
    if diff < 1 || diff > 3 {
      // println!("Unsafe: diff is {0}", diff);
      return false;
    }

    if num_value < last_value {
      inrc -= 1;
    } else if num_value > last_value {
      inrc += 1;
    }
    last_value = num_value;
  }
  println!("separation: {0} - {1}: {2}", inrc.abs(), count - 1, inrc.abs() == count - 1);
  return inrc.abs() == count - 1;
}