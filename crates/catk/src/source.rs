use crate::file::{FileManager, FileRef};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

#[derive(Debug, Clone, Copy)]
pub struct SourceId(pub(crate) u64);

/// Offset within a source file.
#[derive(Debug, Clone, Default)]
pub struct Position(pub usize);

impl From<usize> for Position {
  #[inline]
  fn from(value: usize) -> Self {
    Self(value)
  }
}

impl std::fmt::Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

#[derive(Debug)]
pub struct PositionRef {
  pub source: SourceRef,
  pub position: Position,
}

impl std::fmt::Display for PositionRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

#[derive(Debug, Clone)]
pub struct RegionRef {
  pub source: SourceRef,
  pub begin: Position,
  pub end: Position,
}

impl RegionRef {
  pub fn new(source: SourceRef, begin: impl Into<Position>, end: impl Into<Position>) -> Self {
    Self {
      source,
      begin: begin.into(),
      end: end.into(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Origin {
  Input {
    id: SourceId,
    path: PathBuf,
  },
  IncludedBy {
    parent: SourceId,
    directive_position: Position,
    id: SourceId,
    path: PathBuf,
  },
}

#[derive(Debug, Clone)]
pub struct Source {
  pub origin: Origin,
  pub file: FileRef,
}

impl peg::Parse for Source {
  type PositionRepr = Position;

  #[inline]
  fn start<'input>(&'input self) -> usize {
    0
  }

  #[inline]
  fn is_eof<'input>(&'input self, p: usize) -> bool {
    p >= self.file.inner.content.len()
  }

  #[inline]
  fn position_repr<'input>(&'input self, offset: usize) -> Self::PositionRepr {
    /*
    let before = &self.file.inner.content[..offset];
    let line = before.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1;
    let column = before.chars().rev().take_while(|&c| c != '\n').count() + 1;
    */

    Position(offset)
  }
}

impl peg::ParseLiteral for Source {
  fn parse_string_literal(&self, pos: usize, literal: &str) -> peg::RuleResult<()> {
    self.file.inner.content.parse_string_literal(pos, literal)
  }
}

impl<'input> peg::ParseElem<'input> for Source {
  type Element = char;

  fn parse_elem(&'input self, pos: usize) -> peg::RuleResult<Self::Element> {
    self.file.inner.content.parse_elem(pos)
  }
}

#[derive(Debug, Clone)]
pub struct SourceRef {
  pub id: SourceId,
  pub inner: Rc<Source>,
}

impl peg::Parse for SourceRef {
  type PositionRepr = PositionRef;

  #[inline]
  fn start<'input>(&'input self) -> usize {
    0
  }

  #[inline]
  fn is_eof<'input>(&'input self, p: usize) -> bool {
    p >= self.inner.file.inner.content.len()
  }

  #[inline]
  fn position_repr<'input>(&'input self, offset: usize) -> Self::PositionRepr {
    Self::PositionRepr {
      source: self.clone(),
      position: self.inner.position_repr(offset),
    }
  }
}

impl peg::ParseLiteral for SourceRef {
  fn parse_string_literal(&self, pos: usize, literal: &str) -> peg::RuleResult<()> {
    self.inner.parse_string_literal(pos, literal)
  }
}

impl<'input> peg::ParseElem<'input> for SourceRef {
  type Element = char;

  fn parse_elem(&'input self, pos: usize) -> peg::RuleResult<Self::Element> {
    self.inner.parse_elem(pos)
  }
}

#[derive(Debug, Clone)]
pub struct SourceManager {
  file_manager: FileManager,
  map: Rc<RefCell<HashMap<SourceId, Rc<Source>>>>,
}

impl SourceManager {}
