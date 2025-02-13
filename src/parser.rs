use regex::Regex;

#[derive(Debug)]
pub struct SchemaDefinition {
  pub name: String,
  pub content: String,
}

pub fn parse_tables(schema: &str) -> Vec<SchemaDefinition> {
  // Updated pattern to capture the entire "create_table ... end" snippet in group(1).
  // Capturing group(2) is the quoted table name, group(3) is the symbol name.
  let table_re = Regex::new(r#"(?s)(create_table[\s\(]*(?:"([^"]+)"|:([a-zA-Z0-9_]+)).*?do\s*\|[^|]+\|.*?end)"#).expect("Invalid table regex pattern");

  // View pattern left as-is
  let view_re = Regex::new(r#"(?s)create_view\s*(?:\(\s*)?(?::([a-zA-Z0-9_]+)|"([^"]+)")\s*(?:,|\))?\s*(?:,\s*sql_definition:\s*<<-SQL(.*?)SQL|\{(.*?)\})"#).expect("Invalid view regex pattern");

  let mut tables = Vec::new();

  // Parse tables
  for captures in table_re.captures_iter(schema) {
    // The entire create_table snippet
    let snippet = captures.get(1).unwrap().as_str().to_string();

    // Table name can be in group(2) if quoted, or group(3) if a symbol
    let schema_name = captures.get(2).or_else(|| captures.get(3)).map(|m| m.as_str().to_string()).expect("Missing table name capture");

    tables.push(SchemaDefinition { name: schema_name, content: snippet });
  }

  // Parse views
  for captures in view_re.captures_iter(schema) {
    let schema_name = captures.get(1).or_else(|| captures.get(2)).map(|m| m.as_str().to_string()).expect("Missing view name capture");

    let sql_content = captures.get(3).or_else(|| captures.get(4)).unwrap().as_str();
    let block = format!("create_view \"{}\", sql_definition: <<-SQL{}SQL", schema_name, sql_content);
    tables.push(SchemaDefinition { name: schema_name, content: block });
  }

  tables
}
