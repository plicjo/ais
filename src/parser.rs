use regex::Regex;

#[derive(Debug)]
pub struct SchemaDefinition {
  pub name: String,
  pub content: String,
}

pub fn parse_tables(schema: &str) -> Vec<SchemaDefinition> {
  let table_re = Regex::new(r#"(?s)(create_table)\s+"([^"]+)"\s+do\s+\|[^|]*\|(.*?)end"#).expect("Invalid table regex pattern");
  let view_re = Regex::new(r#"(?s)(create_view)\s+"([^"]+)",\s+sql_definition:\s+<<-SQL(.*?)SQL"#).expect("Invalid view regex pattern");

  let mut tables = Vec::new();

  // Parse tables
  for captures in table_re.captures_iter(schema) {
    let definition_type = captures.get(1).unwrap().as_str();
    let schema_name = captures[2].to_string();
    let block = format!("{} \"{}\" do |t|{}end", definition_type, schema_name, &captures[3]);
    tables.push(SchemaDefinition { name: schema_name, content: block });
  }

  // Parse views
  for captures in view_re.captures_iter(schema) {
    let definition_type = captures.get(1).unwrap().as_str();
    let schema_name = captures[2].to_string();
    let block = format!("{} \"{}\", sql_definition: <<-SQL{}SQL", definition_type, schema_name, &captures[3]);
    tables.push(SchemaDefinition { name: schema_name, content: block });
  }

  tables
}
