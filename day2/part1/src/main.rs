#[path="../../../modules/readdata.rs"]
mod readdata;
mod analysis;

fn main() {
  println!("Day 2 - Part 1");
  let data = readdata::read_input("../input.txt");

  let mut safe_count:u32 = 0;
  let mut unsafe_cout:u32 = 0;
  for line_num in 0..data.len() {
    let line = data[line_num].clone();
    if analysis::analyse(line) {
      safe_count += 1;
    } else {
      unsafe_cout += 1;
    }
  }
  println!("Safe: {0}, Unsafe: {1}", safe_count, unsafe_cout);
}