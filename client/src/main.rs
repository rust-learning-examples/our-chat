use tokio::net::{TcpStream};
use wetalk::Connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 监听指定地址，等待 TCP 连接进来
    let socket = TcpStream::connect("127.0.0.1:5555").await.unwrap();

    let mut connection = Connection::new(socket);
    connection.write_text("Hello, world1!".to_string()).await.unwrap();
    connection.write_text("Hello, world2!".to_string()).await.unwrap();
    connection.write_text("Hello, world3!".to_string()).await.unwrap();

    Ok(())
}