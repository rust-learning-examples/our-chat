use tokio::net::{TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use crate::{Message};

pub struct Connection {
  // socket: TcpStream,
  pub reader: io::ReadHalf<TcpStream>,
  pub writer: io::WriteHalf<TcpStream>,
}

impl Connection {
  pub fn new(socket: TcpStream) -> Self {
    // https://tokio.rs/tokio/tutorial/io
    let (reader, writer) = io::split(socket);
    Connection {
      // socket,
      reader,
      writer,
    }
  }
  pub async fn read_message(&mut self) -> anyhow::Result<Message> {
    let mut size_buf = [0; 8];
    match self.reader.read_exact(&mut size_buf).await {
      Ok(0) => Ok(Message::Close(anyhow::anyhow!("Disconnect with fetch size 0".to_owned()))),
      Ok(_n) => {
        let size = u64::from_be_bytes(size_buf);
        let mut data_buf = vec![0; size.try_into()?];
        match self.reader.read_exact(&mut data_buf).await {
          Ok(0) => Ok(Message::Close(anyhow::anyhow!("Disconnect with fetch size 0".to_owned()))),
          Ok(_n) => {
            let text = String::from_utf8_lossy(&data_buf[..]);
            Ok(Message::Text(text.into()))
          },
          // 非预期错误
          Err(e) => Err(e.into())
        }
      },
      // 非预期错误
      Err(e) => Err(e.into())
    }
  }
  pub async fn write_text(&mut self, text: String) -> anyhow::Result<&mut Self> {
    self.write_message(Message::Text(text)).await
  }
  pub async fn write_message(&mut self, message: Message) -> anyhow::Result<&mut Self> {
    let mut bytes: Option<&[u8]> = None;
    match message {
      Message::Text(ref text) => {
        bytes = Some(text.as_bytes());
      },
      _ => ()
    }

    if let Some(bytes) = bytes {
      let len_bytes = (bytes.len() as u64).to_be_bytes();
      let final_bytes = [len_bytes.as_ref(), bytes].concat();
      self.writer.write_all(&final_bytes).await?;
    }

    Ok(self)
  }
}