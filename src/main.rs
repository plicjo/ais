use clap::Parser;
use std::process;

mod cli;
mod file_ops;
mod parser;

use cli::Cli;
use file_ops::{read_schema_file, write_tables_to_file};
use parser::parse_tables;

fn main() {
  let cli = Cli::parse();

  let contents = match read_schema_file(&cli.schema_path) {
    Ok(s) => s,
    Err(e) => {
      eprintln!("Error reading '{}': {}", cli.schema_path, e);
      process::exit(1);
    }
  };

  let table_definitions = parse_tables(&contents);
  let available_definitions: Vec<_> = table_definitions.iter().map(|td| td.name.clone()).collect();

  let mut successful = Vec::new();
  let mut not_found = Vec::new();

  // Sort requested tables into successful and not found
  for requested_name in &cli.tables {
    if table_definitions.iter().any(|td| &td.name == requested_name) {
      successful.push(requested_name);
    } else {
      not_found.push(requested_name);
    }
  }

  if successful.is_empty() {
    eprintln!("No matching tables or views found. Available tables and views:");
    for name in available_definitions {
      eprintln!("  - {}", name);
    }
    process::exit(1);
  }

  let requested = table_definitions.into_iter().filter(|td| successful.contains(&&td.name)).collect::<Vec<_>>();

  if let Err(e) = write_tables_to_file(&requested, &cli.output_path) {
    eprintln!("Error writing to '{}': {}", cli.output_path, e);
    process::exit(1);
  }

  // Print successful extractions
  println!("Successfully extracted:");
  for name in successful {
    println!("  - {}", name);
  }

  // Print tables/views that weren't found
  if !not_found.is_empty() {
    println!("\nThe following tables/views were not found:");
    for name in not_found {
      println!("  - {}", name);
    }
  }

  println!("\nWrote {} table(s)/view(s) to '{}'.", requested.len(), cli.output_path);
}
