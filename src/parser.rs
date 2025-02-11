use regex::Regex;

#[derive(Debug)]
pub struct SchemaDefinition {
  pub name: String,
  pub content: String,
}

pub fn parse_tables(schema: &str) -> Vec<SchemaDefinition> {
  let re = Regex::new(r#"(?s)(create_table|create_view)\s+"([^"]+)"(.*?)end"#).expect("Invalid regex pattern");

  let mut tables = Vec::new();

  for captures in re.captures_iter(schema) {
    let definition_type = captures.get(1).unwrap().as_str();
    let schema_name = captures[2].to_string();
    let block = format!("{} \"{}\"{}end", definition_type, schema_name, &captures[3]);

    tables.push(SchemaDefinition { name: schema_name, content: block });
  }

  tables
}
