use crate::file::{FileManager, FileRef};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

#[derive(Debug, Clone, Copy)]
pub struct SourceId(pub(crate) u64);

#[derive(Debug, Clone, Default)]
pub struct Position {
  pub line: u32,
  pub column: u32,
  pub offset: usize,
}

impl std::fmt::Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

#[derive(Debug)]
pub struct Location {
  pub source: SourceRef,
  pub position: Position,
}

impl std::fmt::Display for Location {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

#[derive(Debug, Clone)]
pub struct Region {
  pub source: SourceRef,
  pub begin: Position,
  pub end: Position,
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

  fn position_repr<'input>(&'input self, offset: usize) -> Self::PositionRepr {
    let before = &self.file.inner.content[..offset];
    let line = before.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1;
    let column = before.chars().rev().take_while(|&c| c != '\n').count() + 1;

    Self::PositionRepr {
      line: line as u32,
      column: column as u32,
      offset,
    }
  }
}

#[derive(Debug, Clone)]
pub struct SourceRef {
  pub id: SourceId,
  pub inner: Rc<Source>,
}

impl peg::Parse for SourceRef {
  type PositionRepr = Location;

  #[inline]
  fn start<'input>(&'input self) -> usize {
    0
  }

  #[inline]
  fn is_eof<'input>(&'input self, p: usize) -> bool {
    p >= self.inner.file.inner.content.len()
  }

  fn position_repr<'input>(&'input self, offset: usize) -> Self::PositionRepr {
    Self::PositionRepr {
      source: self.clone(),
      position: self.inner.position_repr(offset),
    }
  }
}

#[derive(Debug, Clone)]
pub struct SourceManager {
  file_manager: FileManager,
  map: Rc<RefCell<HashMap<SourceId, Rc<Source>>>>,
}

impl SourceManager {}
