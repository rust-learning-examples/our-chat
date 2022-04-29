
#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("IoError")]
  IO(#[from] std::io::Error),
  #[error("error message: `{0}`")]
  Message(String),
  #[error(transparent)]
  Other(#[from] anyhow::Error),
}