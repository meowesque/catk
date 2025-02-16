use crate::file::{FileManager, FileRef};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

#[derive(Debug, Clone, Copy)]
pub struct SourceId(pub(crate) u64);

#[derive(Debug, Clone, Default)]
pub struct Position {
  pub line: u32,
  pub column: u32,
}

pub struct Location {
  pub source: SourceRef,
  pub position: Position,
}

#[derive(Debug, Clone)]
pub struct Region {
  pub source: SourceRef,
  pub begin: Position,
  pub end: Position,
}

#[derive(Debug, Clone)]
pub struct SourceRef {
  pub id: SourceId,
  pub source: Rc<Source>,
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

#[derive(Debug, Clone)]
pub struct SourceManager {
  file_manager: FileManager,
  map: Rc<RefCell<HashMap<SourceId, Rc<Source>>>>,
}

impl SourceManager {}
