mod options;
mod parser;
mod syntax;

use crate::{context::Context, prelude::*, source};

pub fn expand(
  context: &mut Context,
  options: &options::Options,
  source: source::SourceRef,
) -> Result<syntax::Items> {
  todo!()
}
