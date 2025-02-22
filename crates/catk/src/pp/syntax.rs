use crate::source;
use std::path::PathBuf;

pub enum IncludeLocality {
  Local,
  Global,
}

pub struct IncludePath {
  pub region: source::RegionRef,
  pub path: PathBuf,
  pub locality: IncludeLocality,
}

impl IncludePath {
  pub fn local(region: source::RegionRef, path: PathBuf) -> Self {
    Self {
      region,
      path,
      locality: IncludeLocality::Local,
    }
  }

  pub fn global(region: source::RegionRef, path: PathBuf) -> Self {
    Self {
      region,
      path,
      locality: IncludeLocality::Global,
    }
  }
}

#[derive(Debug)]
pub struct BeforeMacroExpansion {
  pub region: source::RegionRef,
}

#[derive(Debug)]
pub struct AfterMacroExpansion {
  pub region: source::RegionRef,
  pub content: String,
}

#[derive(Debug)]
pub struct BeforeIncludeExpansion {
  pub region: source::RegionRef,
}

#[derive(Debug)]
pub struct AfterIncludeExpansion {}

pub enum Item {
  Annotation {
    region: source::RegionRef,
  },
  Definition {
    region: source::RegionRef,
  },
  IncludeExpansion {
    before: BeforeIncludeExpansion,
    after: AfterIncludeExpansion,
  },
  MacroExpansion {
    before: BeforeMacroExpansion,
    after: AfterMacroExpansion,
  },
  Excerpt {
    region: source::RegionRef,
    content: String,
  },
}

pub struct Items {
  items: Vec<Item>,
}
