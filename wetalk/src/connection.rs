use tokio::net::{TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use crate::types;
use snafu::{prelude::*, Whatever};

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
  pub async fn read_data(&mut self) -> Result<Vec<u8>, Whatever> {
    let mut size_buf = [0; 8];
    match self.reader.read_exact(&mut size_buf).await {
      Ok(0) => whatever!("Disconnect when fetch size"),
      Ok(_n) => {
        let size = u64::from_be_bytes(size_buf);
        let mut data_buf = vec![0; size.try_into().unwrap()];
        match self.reader.read_exact(&mut data_buf).await {
          Ok(0) => whatever!("Disconnect when fetch size"),
          Ok(_n) => {
            Ok(data_buf)
          },
          // 非预期错误，由于我们无需再做什么，因此直接停止处理
          Err(e) => whatever!("{:?}", e),
        }
      },
      // 非预期错误，由于我们无需再做什么，因此直接停止处理
      Err(e) => whatever!("{:?}", e),
    }
  }
  pub async fn write_text(&mut self, text: String) -> Result<&mut Self, Box<dyn std::error::Error>> {
    self.write_data(types::MessageType::Text(text)).await
  }
  pub async fn write_data(&mut self, data: types::MessageType) -> Result<&mut Self, Box<dyn std::error::Error>> {
    let bytes: Option<&[u8]>;
    match data {
      types::MessageType::Text(ref text) => {
        bytes = Some(text.as_bytes());
      },
    }

    if let Some(bytes) = bytes {
      let len_bytes = (bytes.len() as u64).to_be_bytes();
      let final_bytes = [len_bytes.as_ref(), bytes].concat();
      self.writer.write_all(&final_bytes).await?;
    }

    Ok(self)
  }
}