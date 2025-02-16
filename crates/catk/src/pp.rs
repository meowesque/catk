use std::collections::HashMap;

use crate::{
  context::Context,
  prelude::*,
  source::{self, SourceRef},
};

#[derive(Debug)]
pub struct Options {}

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
  Logic {
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

#[derive(Debug, Default)]
struct DefinitionsTable {
  map: HashMap<String, String>,
}

#[derive(Debug)]
struct State<'a> {
  context: &'a mut Context,
  options: &'a Options,
  source: SourceRef,
  position: source::Position,
  offset: usize,
  input: Vec<char>,
  definition_table: DefinitionsTable,
}

impl<'a> State<'a> {
  fn new(context: &'a mut Context, options: &'a Options, source: source::SourceRef) -> Self {
    Self {
      context,
      options,
      source: source.clone(),
      position: source::Position::default(),
      offset: 0,
      input: source.source.file.file.content.chars().collect(),
      definition_table: DefinitionsTable::default(),
    }
  }

  fn cur(&mut self) -> char {
    self.input[self.offset]
  }
}

pub fn expand(
  context: &mut Context,
  options: &Options,
  source: source::SourceRef,
) -> Result<SourceItems> {
  let mut state = State::new(context, options, source);

  todo!()
}
