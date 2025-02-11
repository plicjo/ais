use regex::Regex;

#[derive(Debug)]
pub struct TableDefinition {
  pub name: String,
  pub content: String,
}

pub fn parse_tables(schema: &str) -> Vec<TableDefinition> {
  let re = Regex::new(r#"(?s)create_table\s+"([^"]+)"(.*?)end"#)
    .expect("Invalid regex pattern");

  let mut tables = Vec::new();

  for captures in re.captures_iter(schema) {
    let table_name = captures[1].to_string();
    let block = format!("create_table \"{}\"{}end", table_name, &captures[2]);

    tables.push(TableDefinition {
      name: table_name,
      content: block,
    });
  }

  tables
}
