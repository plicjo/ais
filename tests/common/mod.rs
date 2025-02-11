use std::fs;
use tempfile::tempdir;

use super::TEST_SCHEMA;

pub fn setup_test_schema() -> (tempfile::TempDir, std::path::PathBuf) {
  let dir = tempdir().unwrap();
  let schema_path = dir.path().join("schema.rb");
  fs::write(&schema_path, TEST_SCHEMA).unwrap();
  (dir, schema_path)
}
