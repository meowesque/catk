mod parser;

use crate::{context::Context, prelude::*, source};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Options {
  pub recursion_limit: u32,
}

#[derive(Debug)]
pub struct BeforeExpansion {
  pub region: source::Region,
}

#[derive(Debug)]
pub struct AfterExpansion {
  pub region: source::Region,
  pub content: String,
}

#[derive(Debug)]
pub enum SourceItem {
  Annotation {
    region: source::Region,
  },
  Definition {
    region: source::Region,
  },
  Expansion {
    before: BeforeExpansion,
    after: AfterExpansion,
  },
  Excerpt {
    region: source::Region,
    content: String,
  },
}

#[derive(Debug)]
pub struct SourceItems {
  items: Vec<SourceItem>,
}

pub fn expand(
  context: &mut Context,
  options: &Options,
  source: source::SourceRef,
) -> Result<SourceItems> {
  todo!()
}

#[derive(Debug, Default)]
struct DefinitionsTable {
  map: HashMap<String, String>,
}
