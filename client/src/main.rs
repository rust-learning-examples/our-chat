use tokio::net::{TcpStream};
use wetalk::Connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    // 监听指定地址，等待 TCP 连接进来
    log::info!("connect to: 127.0.0.1:5555");

    match TcpStream::connect("127.0.0.1:5555").await {
        Ok(socket) => {
            let mut connection = Connection::new(socket);
            connection.write_text("Hello, world1!".to_string()).await.unwrap();
            connection.write_text("Hello, world2!".to_string()).await.unwrap();
            connection.write_text("Hello, world3!".to_string()).await.unwrap();
            // 3s后关闭客户端
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
        Err(e) => {
            return Err(e.into())
        }
    }

    Ok(())
}