// use tokio::net::{TcpStream};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};
use futures::{SinkExt, StreamExt};

pub struct ConnectionWs<S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin> {
  // socket: TcpStream,
  pub reader: ReaderConnectionWs<S>,
  pub writer: WriterConnectionWs<S>,
}

impl<S> ConnectionWs<S> where S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin{
  pub fn new(socket: tokio_tungstenite::WebSocketStream<S>) -> Self {
    // https://tokio.rs/tokio/tutorial/io
    let (writer, reader) = socket.split();
    ConnectionWs {
      // socket,
      reader: ReaderConnectionWs::new(reader),
      writer: WriterConnectionWs::new(writer),
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
  pub fn split(self) -> (ReaderConnectionWs<S>, WriterConnectionWs<S>) {
    (self.reader, self.writer)
  }
}

pub struct ReaderConnectionWs<S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin> {
  pub reader: futures::stream::SplitStream<WebSocketStream<S>>,
}

impl<S> ReaderConnectionWs<S> where S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin {
  fn new(reader: futures::stream::SplitStream<WebSocketStream<S>>) -> Self {
    Self { reader }
  }
  pub async fn read_message(&mut self) -> anyhow::Result<Message> {
    if let Some(message) = self.reader.next().await {
      match message {
        Ok(message) => Ok(message),
        Err(err) => Err(err.into()),
      }
    } else {
      Ok(Message::Close(None))
    }
  }
}


pub struct WriterConnectionWs<S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin> {
  pub writer: futures::stream::SplitSink<WebSocketStream<S>, Message>,
}

impl<S> WriterConnectionWs<S> where S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin{
  fn new(writer: futures::stream::SplitSink<WebSocketStream<S>, Message>) -> Self {
    Self { writer }
  }
  pub async fn write_text(&mut self, text: &str) -> anyhow::Result<&mut Self> {
    self.write_message(Message::Text(text.to_string())).await
  }
  pub async fn write_message(&mut self, message: Message) -> anyhow::Result<&mut Self> {
    self.writer.send(message).await?;
    Ok(self)
  }
}