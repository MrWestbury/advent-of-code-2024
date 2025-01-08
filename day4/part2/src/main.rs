#[path="../../../modules/readdata.rs"]
mod readdata;
mod grid;

fn main() {
  println!("Day 4 - Part 2");
  let data = readdata::read_input("../input.txt");

  let mut grid = grid::Grid::new();
  for line_num in 0..data.len() {
    grid.add_row(data[line_num].clone());
  } 
  
  let mut count = 0;
  for item in grid.get_as() {
    if item.x == 0 || item.x == grid.cols-1 || item.y == 0 || item.y == grid.rows - 1 {
      continue;
    }
    count += check(&grid, &item, "MSMS");
    count += check(&grid, &item, "MMSS");
    count += check(&grid, &item, "SMSM");
    count += check(&grid, &item, "SSMM");
  }
  println!("Total: {0}", count);
}

fn check(grid:&grid::Grid, p:&grid::Point, pattern:&str) -> usize {
  let pattern_chars:Vec<char> = pattern.to_string().chars().collect();
  // 1.2
  // .X.
  // 3.4 
  let char1 = grid.get(p.x-1, p.y-1).unwrap();
  let char2 = grid.get(p.x+1, p.y-1).unwrap();
  let char3 = grid.get(p.x-1, p.y+1).unwrap();
  let char4 = grid.get(p.x+1, p.y+1).unwrap();

  if char1 != pattern_chars[0] || char2 != pattern_chars[1] || char3 != pattern_chars[2] || char4 != pattern_chars[3] {
    return 0;
  }
  return 1;
}