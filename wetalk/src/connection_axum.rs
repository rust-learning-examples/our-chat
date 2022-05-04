// use tokio::net::{TcpStream};
use axum::extract::ws::{self, Message};
use futures::{SinkExt, StreamExt};

pub struct ConnectionAxum {
  // socket: TcpStream,
  pub reader: ReaderConnectionAxum,
  pub writer: WriterConnectionAxum,
}

impl ConnectionAxum {
  pub fn new(socket: ws::WebSocket) -> Self {
    // https://tokio.rs/tokio/tutorial/io
    let (writer, reader) = socket.split();
    ConnectionAxum {
      // socket,
      reader: ReaderConnectionAxum::new(reader),
      writer: WriterConnectionAxum::new(writer),
    }
  }
  pub async fn read_message(&mut self) -> anyhow::Result<Message> {
    self.reader.read_message().await
  }
  pub async fn write_text(&mut self, text: &str) -> anyhow::Result<&mut Self> {
    self.writer.write_text(text).await?;
    Ok(self)
  }
  pub async fn write_message(&mut self, message: Message) -> anyhow::Result<&mut Self> {
    self.writer.write_message(message).await?;
    Ok(self)
  }
  pub fn split(self) -> (ReaderConnectionAxum, WriterConnectionAxum) {
    (self.reader, self.writer)
  }
}

pub struct ReaderConnectionAxum {
  pub reader: futures::stream::SplitStream<ws::WebSocket>,
}

impl ReaderConnectionAxum {
  fn new(reader: futures::stream::SplitStream<ws::WebSocket>) -> Self {
    Self { reader }
  }
  pub async fn read_message(&mut self) -> anyhow::Result<Message> {
    if let Some(message) = self.reader.next().await {
      match message {
        Ok(message) => Ok(message.into()),
        Err(err) => Err(err.into()),
      }
    } else {
      Ok(Message::Close(None))
    }
  }
}


pub struct WriterConnectionAxum {
  pub writer: futures::stream::SplitSink<ws::WebSocket, Message>,
}

impl WriterConnectionAxum {
  fn new(writer: futures::stream::SplitSink<ws::WebSocket, Message>) -> Self {
    Self { writer }
  }
  pub async fn write_text(&mut self, text: &str) -> anyhow::Result<&mut Self> {
    self.write_message(Message::Text(text.to_string())).await
  }
  pub async fn write_message(&mut self, message: Message) -> anyhow::Result<&mut Self> {
    self.writer.send(message).await?;
    Ok(self)
  }
  pub async fn close(&mut self) -> anyhow::Result<()> {
    Ok(self.writer.close().await?)
  }
}