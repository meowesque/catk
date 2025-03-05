#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("CATK error: {0}")]
  CatkError(#[from] catk::error::Error),
}
