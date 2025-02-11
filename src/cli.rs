use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ais")]
#[command(about = "Extract table and view definitions from a Rails schema.rb")]
pub struct Cli {
  /// Schema file path (default: db/schema.rb)
  #[arg(short = 'f', long = "file", default_value = "db/schema.rb")]
  pub schema_path: String,

  /// Output file path (default: ai_context_schema.rb)
  #[arg(short = 'o', long = "output", default_value = "ai_context_schema.rb")]
  pub output_path: String,

  /// Table names to extract
  #[arg(required = true)]
  pub tables: Vec<String>,
}
