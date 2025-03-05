use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
  pub sources: Vec<String>,
}