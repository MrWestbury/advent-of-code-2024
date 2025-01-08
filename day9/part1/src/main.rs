#[path="../../../modules/readdata.rs"]
mod readdata;

use std::env;

fn main() {
  println!("Day 9 - Part 1");

  //let mut input_file:String = "../test.txt".to_string();

  // let args: Vec<String> = env::args().collect();
  // match args.len() {
  //   2 => {
  //     input_file = args[1].clone();
  //   },
  //   _ => {}
  // }

  // let data = readdata::read_input(&input_file);
  // let nums:Vec<char> = data[0].chars().collect();
  
  let connection = sqlite::open("./rd.sqlite").unwrap();
  let answer = sum_data(&connection);
  println!("Answer: {0}", answer);
  // create_table(&connection);
  // let query_raw = "INSERT INTO filesystem VALUES(?, ?);";
  // let mut query = connection.prepare(query_raw).unwrap();

  // let mut address:i64 = 0;
  // let mut file_id:i64 = 0;
  // for i in 0..nums.len() {
  //   let actual_num:i32 = format!("{0}", nums[i]).parse().unwrap();
  //   for _offet in 0..actual_num {
  //     if i % 2 == 0 {
  //       let _ = query.bind((1, address));
  //       let _ = query.bind((2, file_id));
  //     } else {
  //       let _ = query.bind((1, address)).unwrap();
  //       let _ = query.bind((2, None::<i64>)).unwrap();
  //     }
  //     let _ = query.next();
  //     let _ = query.reset();
  //     address += 1;
  //   }
  //   if i % 2 == 0 {
  //     file_id += 1;
  //   }
  // }

  // sort_table(&connection);
}

fn create_table(conn:&sqlite::Connection) {
  // let create_table_query = "CREATE TABLE IF NOT EXISTS filesystem (address INTEGER, fileid INTEGER); DELETE FROM filesystem;";
  //conn.execute(create_table_query).unwrap();
}

fn sort_table(conn:&sqlite::Connection) {
  let mut free_addr = get_lowest_free_address(conn);
  let mut file_info = get_highest_fileid(conn);
  while free_addr < file_info.0 {
    //println!("Swap: free: {0}, file_addr: {1}, file_id: {2}", free_addr, file_info.0, file_info.1);
    swap_fileblock(conn, free_addr, file_info.0, file_info.1);
    free_addr = get_lowest_free_address(conn);
    file_info = get_highest_fileid(conn);
  }
}

fn get_lowest_free_address(conn:&sqlite::Connection) -> i64 {
  let lowest_empty_address_query = "SELECT address FROM filesystem WHERE fileid IS NULL ORDER BY address ASC LIMIT 1;";
  
  let mut stmt = conn.prepare(lowest_empty_address_query).unwrap();
  let _ = stmt.next();
  let low_addr:i64 = stmt.read::<i64, _>("address").unwrap();
  return low_addr;
}

fn get_highest_fileid(conn:&sqlite::Connection) -> (i64, i64) {
  let highest_file_query = "SELECT address, fileid FROM filesystem WHERE fileid IS NOT NULL ORDER BY address DESC LIMIT 1;";
  let mut stmt = conn.prepare(highest_file_query).unwrap();
  let _ = stmt.next();
  let high_addr = stmt.read::<i64, _>("address").unwrap();
  let high_fileid = stmt.read::<i64, _>("fileid").unwrap();
  return (high_addr, high_fileid);
}

fn swap_fileblock(conn:&sqlite::Connection, empty_block_address:i64, file_block_address:i64, file_id:i64) {
  let swap_query = "UPDATE filesystem SET fileid = ? WHERE address = ?;";
  let mut stmt = conn.prepare(swap_query).unwrap();
  let _ = stmt.bind((1, file_id)).unwrap();
  let _ = stmt.bind((2, empty_block_address)).unwrap();
  let _ = stmt.next();

  let null_query = "UPDATE filesystem SET fileid = null WHERE address = ?;";
  stmt = conn.prepare(null_query).unwrap();
  let _ = stmt.bind((1, file_block_address)).unwrap();
  let _ = stmt.next();
}

fn sum_data(conn:&sqlite::Connection) -> i64 {
  let query = "SELECT SUM(address * fileid) as answer FROM filesystem WHERE fileid IS NOT NULL;";
  let mut stmt = conn.prepare(query).unwrap();
  let _ = stmt.next();
  let answer = stmt.read::<i64,_>("answer").unwrap();
  return answer;
}