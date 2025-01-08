#[derive(PartialEq)]
pub struct FileRecord {
  pub start_address:i64,
  pub end_address:i64,
  pub file_id:Option<i64>,
  pub size:i64
}

impl FileRecord {
  pub fn new(start_address:i64, block_size:i64) -> FileRecord {
    FileRecord {
      start_address: start_address,
      end_address: start_address + block_size - 1,
      file_id: None::<i64>,
      size: block_size
    }
  }

  pub fn new_with_file_id(start_address:i64, block_size:i64, file_id:i64) -> FileRecord {
    FileRecord {
      start_address: start_address,
      end_address: start_address + block_size - 1,
      file_id: Some(file_id),
      size: block_size
    }
  }

  pub fn bind(&self, stmt:&mut sqlite::Statement) {
    stmt.bind((":start_addr", self.start_address)).unwrap();
    stmt.bind((":end_addr", self.end_address)).unwrap();
    stmt.bind((":file_id", self.file_id)).unwrap();
    stmt.bind((":size", self.size)).unwrap();
  }
}