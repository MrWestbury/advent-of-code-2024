#[derive(PartialEq)]
enum TrendKind {
  Unknown,
  Increasing,
  Decreasing,
}

pub fn analyse(data:&String) -> bool {
  let mut debug_msg:String = String::new();
  let iter = data.split_whitespace();
  let num_vec:Vec<i32> = iter.map(|str| str.parse().unwrap()).collect();
  let trend:TrendKind = get_trend(&num_vec);
  let mut bad_count = 0;
  for idx in 1..num_vec.len() {
    let num_pair_1 = num_vec[idx-1];
    let num_pair_2 = num_vec[idx];

    let result = is_bad_value(&trend, num_pair_1, num_pair_2);
    if !result {
      bad_count += 1;
    }
  }

  return bad_count <= 1;
}

fn trend_to_str(trend:TrendKind) -> String {
  return (match trend {
    TrendKind::Unknown => "Unknown",
    TrendKind::Increasing => "Increasing",
    TrendKind::Decreasing => "Decreasing",
  }).to_string();
}

fn get_trend(data:&Vec<i32>) -> TrendKind {
  let mut trend_marker = 0;
  let mut previous:i32 = -1;
  for item in data {
    if previous == -1 {
      previous = *item;
      continue;
    }

    if *item < previous {
      trend_marker -= 1;
    } else if *item > previous {
      trend_marker += 1;
    }
  }

  if trend_marker > 0 {
    return TrendKind::Increasing;
  } else if trend_marker < 0 {
    return TrendKind::Decreasing;
  }
  return TrendKind::Unknown;
}

fn is_bad_value(trend:&TrendKind, last:i32, current:i32) -> bool {
  /// Checks if a pair of values are bad
  if last == current {
    return true;
  }
  if *trend == TrendKind::Increasing && (last >= current || (current - last) > 3) {
    return true;
  } else if *trend == TrendKind::Decreasing && (last <= current || (last - current) > 3) {
    return true;
  }
  return false;
}