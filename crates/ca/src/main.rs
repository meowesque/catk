use clap::Parser;

mod cli;
mod error;
mod prelude;

fn main() -> prelude::Result<()> {
  let args = cli::Args::parse();

  todo!()
}
