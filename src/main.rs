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

  let available_tables: Vec<_> = table_definitions.iter().map(|td| td.name.clone()).collect();

  let requested = table_definitions.into_iter().filter(|td| cli.tables.contains(&td.name)).collect::<Vec<_>>();

  if requested.is_empty() {
    eprintln!("No matching tables found. Available tables:");
    for table_name in available_tables {
      eprintln!("  - {}", table_name);
    }
    process::exit(1);
  }

  if let Err(e) = write_tables_to_file(&requested, &cli.output_path) {
    eprintln!("Error writing to '{}': {}", cli.output_path, e);
    process::exit(1);
  }

  println!("Successfully wrote {} table(s) to '{}'.", requested.len(), cli.output_path);
}
