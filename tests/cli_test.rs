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

#[test]
fn test_mixed_existing_and_nonexistent_tables() {
  let (dir, schema_path) = common::setup_test_schema();

  // Read and parse
  let contents = read_schema_file(schema_path.to_str().unwrap()).unwrap();
  let tables = parse_tables(&contents);

  // Request mix of existing and non-existing tables
  let requested_tables = vec!["contacts".to_string(), "nonexistent".to_string()];
  let filtered: Vec<_> = tables.into_iter().filter(|td| requested_tables.contains(&td.name)).collect();

  // Verify we only got the existing table
  assert_eq!(filtered.len(), 1);
  assert_eq!(filtered[0].name, "contacts");

  // Write output
  let output_path = dir.path().join("output.rb");
  write_tables_to_file(&filtered, output_path.to_str().unwrap()).unwrap();

  // Verify output only contains the existing table
  let output_content = fs::read_to_string(output_path).unwrap();
  assert!(output_content.contains("create_table \"contacts\""));
  assert!(!output_content.contains("nonexistent"));
}

#[test]
fn test_list_available_tables() {
  let (_dir, schema_path) = common::setup_test_schema();

  // Read and parse
  let contents = read_schema_file(schema_path.to_str().unwrap()).unwrap();
  let tables = parse_tables(&contents);

  // Verify all expected tables and views are present
  let table_names: Vec<_> = tables.iter().map(|td| td.name.clone()).collect();
  assert!(table_names.contains(&"contacts".to_string()));
  assert!(table_names.contains(&"matters".to_string()));
  assert!(table_names.contains(&"notes".to_string()));
  assert!(table_names.contains(&"charges".to_string()));

  // Verify the exact number of tables/views
  assert_eq!(table_names.len(), 4, "Expected exactly 4 tables/views");
}

#[test]
fn test_no_matching_tables() {
  let (_dir, schema_path) = common::setup_test_schema();

  // Read and parse
  let contents = read_schema_file(schema_path.to_str().unwrap()).unwrap();
  let tables = parse_tables(&contents);

  // Request only non-existing tables
  let requested_tables = vec!["nonexistent1".to_string(), "nonexistent2".to_string()];
  let filtered: Vec<_> = tables.into_iter().filter(|td| requested_tables.contains(&td.name)).collect();

  // Verify we got no tables
  assert_eq!(filtered.len(), 0);
}

#[test]
fn test_extract_single_view() {
  let (dir, schema_path) = common::setup_test_schema();

  // Read and parse
  let contents = read_schema_file(schema_path.to_str().unwrap()).unwrap();
  let tables = parse_tables(&contents);

  // Filter for just the view
  let requested_views = vec!["charges".to_string()];
  let filtered: Vec<_> = tables.into_iter().filter(|td| requested_views.contains(&td.name)).collect();

  // Verify we got exactly one view
  assert_eq!(filtered.len(), 1, "Expected exactly one view");
  assert_eq!(filtered[0].name, "charges");

  // Write output
  let output_path = dir.path().join("output.rb");
  write_tables_to_file(&filtered, output_path.to_str().unwrap()).unwrap();

  // Verify output contains only the view
  let output_content = fs::read_to_string(output_path).unwrap();
  assert!(output_content.contains("create_view \"charges\""));
  assert!(!output_content.contains("create_table"));
}

#[test]
fn test_parse_table_with_schema_variations() {
  let test_schema = r#"ActiveRecord::Schema[7.0].define(version: 2025_02_11_123456) do
        create_table "contacts" do |t|
            t.string "email"
        end

        create_table(:users, force: :cascade) do |t|
            t.string "name"
        end

        create_table "members", force: :cascade do |t|
            t.string "title"
        end

        create_table(:tasks) do |t|
            t.string "description"
        end
    end"#;

  let (_dir, schema_path) = common::setup_test_schema();
  fs::write(&schema_path, test_schema).unwrap();

  // Read and parse
  let contents = read_schema_file(schema_path.to_str().unwrap()).unwrap();
  let tables = parse_tables(&contents);

  // Verify all table variations are parsed correctly
  let table_names: Vec<_> = tables.iter().map(|td| td.name.clone()).collect();
  assert!(table_names.contains(&"contacts".to_string()));
  assert!(table_names.contains(&"users".to_string()));
  assert!(table_names.contains(&"members".to_string()));
  assert!(table_names.contains(&"tasks".to_string()));
}
