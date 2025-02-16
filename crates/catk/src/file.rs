use crate::prelude::*;
use std::{
  collections::HashSet,
  path::{Path, PathBuf},
  rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Options {
  pub include_directories: HashSet<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct File {
  pub path: PathBuf,
  pub content: String,
}

#[derive(Debug, Clone)]
pub struct FileId(pub(crate) u64);

#[derive(Debug, Clone)]
pub struct FileRef {
  pub id: FileId,
  pub file: Rc<File>,
}

#[derive(Debug, Clone)]
pub struct FileManager {
  options: Rc<Options>,
}

impl FileManager {
  pub fn new(options: Options) -> Self {
    Self {
      options: Rc::new(options),
    }
  }

  pub fn lookup_local_file(&self, path: impl AsRef<Path>) -> Result<Option<FileRef>> {
    todo!()
  }

  pub fn lookup_global_file(&self, path: impl AsRef<Path>) -> Result<Option<FileRef>> {
    todo!()
  }
}
