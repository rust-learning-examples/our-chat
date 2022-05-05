use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::net::{TcpStream};
use wetalk::Connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // pretty_env_logger::init();
    // 设置 `tracing-subscriber` 对 tracing 数据的处理方式
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).init();
    // 监听指定地址，等待 TCP 连接进来
    log::info!("connect to: 127.0.0.1:5555");

    match TcpStream::connect("127.0.0.1:5555").await {
        Ok(socket) => {
            let mut connection = Connection::new(socket);
            connection.write_text("Hello, world1!").await.unwrap();
            log::debug!("send text message: Hello, world1!");
            connection.write_text("Hello, world2!").await.unwrap();
            log::debug!("send text message: Hello, world12!");
            connection.write_text("Hello, world3!").await.unwrap();
            log::debug!("send text message: Hello, world13!");
            loop {
                match connection.read_message().await {
                    Ok(message) => {
                        match message {
                            wetalk::TSMessage::Text(message) => {
                                log::debug!("recv text message: {}", message);
                            },
                            wetalk::TSMessage::Close(close_frame) => {
                                log::debug!("server close connected, err: {:?}", close_frame);
                                return Err(anyhow::anyhow!("server close connected"))
                            },
                            _ => ()
                        }
                    },
                    Err(err) => {
                        log::debug!("server close connected, err: {:?}", err);
                        return Err(err)
                    }
                }
            }
            // 3s后关闭客户端
            // std::thread::sleep(std::time::Duration::from_secs(3));
            // tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
        Err(e) => {
            return Err(e.into())
        }
    }

    // Ok(())
}