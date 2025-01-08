#[path="../../../modules/readdata.rs"]
mod readdata;
mod file;

use std::env;
use file::FileRecord;
use sqlite::State;

fn main() {
  println!("Day 9 - Part 2");
  let mut input_file:String = "../test.txt".to_string();

  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
    _ => {}
  }

  let data = readdata::read_input(&input_file);
  let nums:Vec<char> = data[0].chars().collect();

  let connection = sqlite::open(":memory:").unwrap();
  print!("Creating table: ");
  create_table(&connection);
  println!("Done");

  print!("Populating data: ");
  populate_db(&connection, &nums);
  println!("Done");

  print!("Swap data: ");
  defrag(&connection);
  println!("Done");

  println!("Answer: {0}", calculate_checksum(&connection));
}

fn create_table(conn:&sqlite::Connection) {
  let create_table_query = "CREATE TABLE IF NOT EXISTS filesystem (start_address INTEGER NOT NULL PRIMARY KEY, end_address INTEGER NOT NULL, file_id INTEGER, block_size INTEGER NOT NULL, fixed INTEGER); DELETE FROM filesystem;";
  conn.execute(create_table_query).unwrap();
}

fn populate_db(conn:&sqlite::Connection, nums:&Vec<char>) {
  conn.execute("BEGIN TRANSACTION;").unwrap();
  let mut address:i64 = 0;
  let mut file_id:i64 = 0;
  for i in 0..nums.len() {
    let actual_num:i64 = format!("{0}", nums[i]).parse().unwrap();
    let fr:FileRecord;
    if i % 2 == 0 {
      // Is a file
      fr = FileRecord::new_with_file_id(address, actual_num, file_id);
      file_id += 1;
    } else {
      fr = FileRecord::new(address, actual_num);
    }
    // let _ = fr.bind(&mut query);
    if actual_num > 0 {
      insert_file(&conn, &fr);
    }
    // let _ = query.reset();
    address += actual_num;
  }
  conn.execute("COMMIT;").unwrap();
}

fn defrag(conn:&sqlite::Connection) {
  let mut last_file:FileRecord = get_last_file(&conn);
  let mut space = find_space(&conn, last_file.size);
  while last_file.end_address != -1 {
    if space.end_address != -1 && space.start_address < last_file.start_address{
      swap_file(&conn, last_file, space.try_into().unwrap());
    } else {
      mark_file_as_fixed(&conn, &last_file);
    }
    last_file = get_last_file(&conn);
    space = find_space(&conn, last_file.size);
  }
  
}

fn get_last_file(conn:&sqlite::Connection) -> FileRecord {
  let query_raw = "SELECT * FROM filesystem WHERE file_id IS NOT NULL AND fixed = 0 ORDER BY start_address DESC LIMIT 1;";
  let mut stmt = conn.prepare(query_raw).unwrap();
  let _ = stmt.next();
  let start_addr:i64 = stmt.read::<i64, _>("start_address").unwrap();
  let size:i64 = stmt.read::<i64, _>("block_size").unwrap();
  let file_id:i64 = stmt.read::<i64, _>("file_id").unwrap();
  let fr = FileRecord::new_with_file_id(start_addr, size, file_id);
  return fr;
}

fn find_space(conn:&sqlite::Connection, min_size:i64) -> FileRecord {
  let query_raw = "SELECT * FROM filesystem WHERE file_id IS NULL AND block_size >= :min_size ORDER BY start_address ASC LIMIT 1;";
  let mut stmt = conn.prepare(query_raw).unwrap();
  stmt.bind((":min_size", min_size)).unwrap();
  let _ = stmt.next();
  let start_addr:i64 = stmt.read::<i64, _>("start_address").unwrap();
  let size:i64 = stmt.read::<i64, _>("block_size").unwrap();
  let fr = FileRecord::new(start_addr, size);
  return fr;
}

fn swap_file(conn:&sqlite::Connection, src:FileRecord, dst:FileRecord) {
  conn.execute("BEGIN TRANSACTION;").unwrap();
  // Delete the empty space
  let delete_query = "DELETE FROM filesystem WHERE start_address = :start_addr;";
  let mut del_stmt = conn.prepare(delete_query).unwrap();
  del_stmt.bind((":start_addr", dst.start_address)).unwrap();
  let _ = del_stmt.next();

  // Set source file record to empty
  let update_src_query = "UPDATE filesystem SET file_id = NULL WHERE start_address = :src_addr;";
  let mut stmt_src = conn.prepare(update_src_query).unwrap();
  stmt_src.bind((":src_addr", src.start_address)).unwrap();
  let _ = stmt_src.next();

  // Copy file record into space
  let new_file = FileRecord::new_with_file_id(dst.start_address, src.size, src.file_id.unwrap());
  insert_file(&conn, &new_file);
  mark_file_as_fixed(&conn, &new_file);

  // Fill in spare empty space
  if dst.size > src.size {
    let empty_space = FileRecord::new(dst.start_address + src.size, dst.size - src.size);
    
    insert_file(&conn, &empty_space);
  }
  
  conn.execute("COMMIT;").unwrap();
}

fn insert_file(conn:&sqlite::Connection, fr:&FileRecord) {
  let query_raw = "INSERT INTO filesystem VALUES (:start_addr, :end_addr, :file_id, :size, 0);";
  let mut insert_stmt = conn.prepare(query_raw).unwrap();
  fr.bind(&mut insert_stmt);
  insert_stmt.next().unwrap();
}

fn mark_file_as_fixed(conn:&sqlite::Connection, fr:&FileRecord) {
  let update_src_query = "UPDATE filesystem SET fixed = 1 WHERE start_address = :src_addr;";
  let mut stmt_src = conn.prepare(update_src_query).unwrap();
  stmt_src.bind((":src_addr", fr.start_address)).unwrap();
  let _ = stmt_src.next();
}

fn calculate_checksum(conn:&sqlite::Connection) -> i64 {
  let query_raw = "SELECT * FROM filesystem WHERE file_id IS NOT NULL;";
  let mut stmt = conn.prepare(query_raw).unwrap();
  let mut total:i64 = 0;
  while let Ok(State::Row) = stmt.next() {
    let start_addr:i64 = stmt.read::<i64, _>("start_address").unwrap();
    let size:i64 = stmt.read::<i64, _>("block_size").unwrap();
    let file_id:i64 = stmt.read::<i64, _>("file_id").unwrap();
    for idx in start_addr..(start_addr+size) {
      total += file_id * idx;
    }
  }
  return total;
}