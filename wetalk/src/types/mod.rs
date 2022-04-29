mod error;
pub use error::Error;

pub enum MessageType {
  Text(String),
}