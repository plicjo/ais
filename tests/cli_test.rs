use ais::{file_ops::*, parser::parse_tables};
use std::fs;
use tempfile::tempdir;

pub const TEST_SCHEMA: &str = r#"ActiveRecord::Schema[7.0].define(version: 2024_02_11_123456) do
  create_table "contacts" do |t|
    t.string "email"
    t.string "first_name"
    t.string "last_name"
    t.timestamps
  end

  create_table "matters" do |t|
    t.string "title"
    t.text "description"
    t.references "contact", null: false
    t.timestamps
  end

  create_table "notes" do |t|
    t.text "content"
    t.references "matter", null: false
    t.timestamps
  end

  create_view "charges", sql_definition: <<-SQL
      SELECT time_entries.actual_hours
      FROM time_entries;
  SQL
end"#;

mod common;

#[test]
fn test_extract_specific_tables() {
  let (dir, schema_path) = common::setup_test_schema();

  // Read and parse
  let contents = read_schema_file(schema_path.to_str().unwrap()).unwrap();
  let tables = parse_tables(&contents);

  // Filter tables
  let requested_tables = vec!["contacts".to_string(), "matters".to_string()];
  let filtered: Vec<_> = tables.into_iter().filter(|td| requested_tables.contains(&td.name)).collect();

  // Write output
  let output_path = dir.path().join("output.rb");
  write_tables_to_file(&filtered, output_path.to_str().unwrap()).unwrap();

  // Verify output
  let output_content = fs::read_to_string(output_path).unwrap();
  assert!(output_content.contains("create_table \"contacts\""));
  assert!(output_content.contains("create_table \"matters\""));
  assert!(!output_content.contains("create_table \"notes\""));
}

#[test]
fn test_extract_specific_views() {
  let (dir, schema_path) = common::setup_test_schema();

  // Read and parse
  let contents = read_schema_file(schema_path.to_str().unwrap()).unwrap();
  let tables = parse_tables(&contents);

  // Filter tables - in this case, views are treated the same as tables
  let requested_views = vec!["charges".to_string()];
  let filtered: Vec<_> = tables.into_iter().filter(|td| requested_views.contains(&td.name)).collect();

  // Write output
  let output_path = dir.path().join("output.rb");
  write_tables_to_file(&filtered, output_path.to_str().unwrap()).unwrap();

  // Verify output
  let output_content = fs::read_to_string(output_path).unwrap();
  assert!(output_content.contains("create_view \"charges\""));
}

#[test]
fn test_cli_error_handling() {
  let dir = tempdir().unwrap();
  let nonexistent_path = dir.path().join("nonexistent.rb");

  // Test reading non-existent file
  let result = read_schema_file(nonexistent_path.to_str().unwrap());
  assert!(result.is_err());
}
