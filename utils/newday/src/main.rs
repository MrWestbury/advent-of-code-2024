use std::io;
use std::io::Write;
use std::fs::File;
use std::fs;

fn main() -> io::Result<()> {
  print!("Enter day number: ");
  let _ = io::stdout().flush();
  let mut day_num_buffer = String::new();
  io::stdin().read_line(&mut day_num_buffer)?;
  day_num_buffer = day_num_buffer.trim().to_string();
  
  print!("Enter part number: ");
  let _ = io::stdout().flush();
  let mut part_num_buffer = String::new();
  io::stdin().read_line(&mut part_num_buffer)?;
  part_num_buffer = part_num_buffer.trim().to_string();
  
  let full_path = format!("day{0}/part{1}/src", day_num_buffer.trim(), part_num_buffer.trim());
  fs::create_dir_all(full_path)?;

  let _ = create_main(&day_num_buffer, &part_num_buffer);
  let _ = create_cargo(&day_num_buffer, &part_num_buffer);
  let _ = create_inputs(&day_num_buffer);
  Ok(())
}

fn create_main(day_num:&String, part_num:&String) -> Result<(), Box<dyn std::error::Error>> {
  let full_path = format!("day{0}/part{1}/src/main.rs", day_num, part_num);
  let mut fs = File::create(full_path)?;
  fs.write(b"#[path=\"../../../modules/readdata.rs\"]\n")?;
  fs.write(b"mod readdata;\n\n")?;
  fs.write(b"use std::env;\n\n")?;
  fs.write(b"fn main() {\n")?;
  let title_str = format!("  println!(\"Day {0} - Part {1}\");\n\n", day_num, part_num);
  fs.write(&title_str.into_bytes())?;
  fs.write(b"  let mut input_file:String = \"../test.txt\".to_string();\n\n")?;
  fs.write(b"  let args: Vec<String> = env::args().collect();\n")?;
  fs.write(b"  match args.len() {\n")?;
  fs.write(b"    2 => {\n")?;
  fs.write(b"      input_file = args[1].clone();\n")?;
  fs.write(b"    },\n")?;
  fs.write(b"  _ => {}\n")?;
  fs.write(b"  }\n\n")?;
  fs.write(b"  let data = readdata::read_input(&input_file);\n\n")?;
  fs.write(b"  // ENTER CODE HERE\n")?;
  fs.write(b"}\n")?;

  Ok(())
}

fn create_cargo(day_num:&String, part_num:&String) -> Result<(), Box<dyn std::error::Error>> {
  let full_path = format!("day{0}/part{1}/Cargo.toml", day_num, part_num);
  let mut fs = File::create(full_path)?;
  let app_name = format!("day{0}part{1}", day_num, part_num);

  fs.write(b"[package]\n")?;
  fs.write(&app_name.into_bytes())?;
  fs.write(b"version = \"0.1.0\"\n")?;
  fs.write(b"edition = \"2021\"\n\n")?;
  fs.write(b"[dependencies]")?;
  Ok(())
}

fn create_inputs(day_num:&String) -> Result<(), Box<dyn std::error::Error>> {
  let full_path = format!("day{0}/test.txt", day_num);
  if !fs::exists(&full_path).unwrap() {
    let _fs = File::create(full_path)?;
  }
  Ok(())
}
