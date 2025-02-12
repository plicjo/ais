use regex::Regex;

#[derive(Debug)]
pub struct SchemaDefinition {
  pub name: String,
  pub content: String,
}

pub fn parse_tables(schema: &str) -> Vec<SchemaDefinition> {
  // Pattern that handles full table definitions with options
  let table_re = Regex::new(r#"(?s)create_table(?:\s*\()?\s*(?:"([^"]+)"|:([a-zA-Z0-9_]+))(?:[^|]*?force:\s*:cascade[^|]*?|\s*\))?\s*do\s*\|[^|]*\|(.*?)end"#).expect("Invalid table regex pattern");

  let view_re = Regex::new(r#"(?s)create_view\s*(?:\(\s*)?(?::([a-zA-Z0-9_]+)|"([^"]+)")\s*(?:,|\))?\s*(?:,\s*sql_definition:\s*<<-SQL(.*?)SQL|\{(.*?)\})"#).expect("Invalid view regex pattern");

  let mut tables = Vec::new();

  // Parse tables with improved capture group handling
  for captures in table_re.captures_iter(schema) {
    // Get name from symbol, string or direct name capture group
    let schema_name = captures
      .get(1)
      .or_else(|| captures.get(2))
      .or_else(|| captures.get(3))
      .map(|m| m.as_str().to_string())
      .expect("Missing table name capture");

    let block = format!("create_table \"{}\" do |t|{}end", schema_name, captures.get(3).unwrap().as_str());
    tables.push(SchemaDefinition { name: schema_name, content: block });
  }

  // Parse views with similar capture group handling
  for captures in view_re.captures_iter(schema) {
    // Get name from either symbol or string capture group
    let schema_name = captures.get(1).or_else(|| captures.get(2)).map(|m| m.as_str().to_string()).expect("Missing view name capture");

    let sql_content = captures.get(3).or_else(|| captures.get(4)).unwrap().as_str();
    let block = format!("create_view \"{}\", sql_definition: <<-SQL{}SQL", schema_name, sql_content);
    tables.push(SchemaDefinition { name: schema_name, content: block });
  }

  tables
}
