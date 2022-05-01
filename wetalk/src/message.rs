pub enum Message {
  Text(String),
  Binary(Vec<u8>),
  Ping(String),
  Pong(String),
  Close(anyhow::Error),
}