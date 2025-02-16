use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Context {
  pub(crate) symbols: lasso::Rodeo,
}
