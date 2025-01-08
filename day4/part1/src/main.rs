#[path="../../../modules/readdata.rs"]
mod readdata;
mod grid;

fn main() {
  println!("Day 4 - Part 1");
  let data = readdata::read_input("../input.txt");

  let mut grid = grid::Grid::new();
  for line_num in 0..data.len() {
    grid.add_row(data[line_num].clone());
  } 
  
  let mut count = 0;
  for item in grid.get_xs() {
    count += check_up_left(&grid, &item);
    count += check_up(&grid, &item);
    count += check_up_right(&grid, &item);
    count += check_right(&grid, &item);
    count += check_down_right(&grid, &item);
    count += check_down(&grid, &item);
    count += check_down_left(&grid, &item);
    count += check_left(&grid, &item);
  }
  println!("Total: {0}", count);
}

fn check_up_left(grid:&grid::Grid, p:&grid::Point) -> usize {
  if p.x < 3 {
    return 0;
  }
  if p.y < 3 {
    return 0;
  }
  
  let char1 = grid.get(p.x, p.y).unwrap();
  let char2 = grid.get(p.x-1, p.y-1).unwrap();
  let char3 = grid.get(p.x-2, p.y-2).unwrap();
  let char4 = grid.get(p.x-3, p.y-3).unwrap();

  if char1 != 'X' || char2 != 'M' || char3 != 'A' || char4 != 'S' {
    return 0;
  }
  return 1;
}

fn check_up(grid:&grid::Grid, p:&grid::Point) -> usize {
  if p.y < 3 {
    return 0;
  }
  
  let char1 = grid.get(p.x, p.y).unwrap();
  let char2 = grid.get(p.x, p.y-1).unwrap();
  let char3 = grid.get(p.x, p.y-2).unwrap();
  let char4 = grid.get(p.x, p.y-3).unwrap();

  if char1 != 'X' || char2 != 'M' || char3 != 'A' || char4 != 'S' {
    return 0;
  }
  return 1;
}

fn check_up_right(grid:&grid::Grid, p:&grid::Point) -> usize {
  if p.x > grid.cols - 4 {
    return 0;
  }
  if p.y < 3 {
    return 0;
  }
  
  let char1 = grid.get(p.x, p.y).unwrap();
  let char2 = grid.get(p.x+1, p.y-1).unwrap();
  let char3 = grid.get(p.x+2, p.y-2).unwrap();
  let char4 = grid.get(p.x+3, p.y-3).unwrap();

  if char1 != 'X' || char2 != 'M' || char3 != 'A' || char4 != 'S' {
    return 0;
  }
  return 1;
}

fn check_left(grid:&grid::Grid, p:&grid::Point) -> usize {
  if p.x < 3 {
    return 0;
  }
  
  let char1 = grid.get(p.x, p.y).unwrap();
  let char2 = grid.get(p.x-1, p.y).unwrap();
  let char3 = grid.get(p.x-2, p.y).unwrap();
  let char4 = grid.get(p.x-3, p.y).unwrap();

  if char1 != 'X' || char2 != 'M' || char3 != 'A' || char4 != 'S' {
    return 0;
  }
  return 1;
}

fn check_right(grid:&grid::Grid, p:&grid::Point) -> usize {
  if p.x > grid.cols - 4 {
    return 0;
  }
  
  let char1 = grid.get(p.x, p.y).unwrap();
  let char2 = grid.get(p.x+1, p.y).unwrap();
  let char3 = grid.get(p.x+2, p.y).unwrap();
  let char4 = grid.get(p.x+3, p.y).unwrap();

  if char1 != 'X' || char2 != 'M' || char3 != 'A' || char4 != 'S' {
    return 0;
  }
  return 1;
}

fn check_down_right(grid:&grid::Grid, p:&grid::Point) -> usize {
  if p.x > grid.rows - 4 {
    return 0;
  }
  if p.y > grid.cols - 4 {
    return 0;
  }
  
  let char1 = grid.get(p.x, p.y).unwrap();
  let char2 = grid.get(p.x+1, p.y+1).unwrap();
  let char3 = grid.get(p.x+2, p.y+2).unwrap();
  let char4 = grid.get(p.x+3, p.y+3).unwrap();

  if char1 != 'X' || char2 != 'M' || char3 != 'A' || char4 != 'S' {
    return 0;
  }
  return 1;
}

fn check_down(grid:&grid::Grid, p:&grid::Point) -> usize {
  if p.y > grid.cols - 4 {
    return 0;
  }
  
  let char1 = grid.get(p.x, p.y).unwrap();
  let char2 = grid.get(p.x, p.y+1).unwrap();
  let char3 = grid.get(p.x, p.y+2).unwrap();
  let char4 = grid.get(p.x, p.y+3).unwrap();

  if char1 != 'X' || char2 != 'M' || char3 != 'A' || char4 != 'S' {
    return 0;
  }
  return 1;
}

fn check_down_left(grid:&grid::Grid, p:&grid::Point) -> usize {
  if p.x < 3 {
    return 0;
  }
  if p.y > grid.cols - 4 {
    return 0;
  }
  
  let char1 = grid.get(p.x, p.y).unwrap();
  let char2 = grid.get(p.x-1, p.y+1).unwrap();
  let char3 = grid.get(p.x-2, p.y+2).unwrap();
  let char4 = grid.get(p.x-3, p.y+3).unwrap();

  if char1 != 'X' || char2 != 'M' || char3 != 'A' || char4 != 'S' {
    return 0;
  }
  return 1;
}