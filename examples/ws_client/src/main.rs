use wetalk::tokio_tungstenite;
use wetalk::ConnectionWs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // pretty_env_logger::init();

    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
    // 监听指定地址，等待 TCP 连接进来
    log::info!("connect to: 127.0.0.1:5555");
    let url = url::Url::parse("ws://127.0.0.1:5555").unwrap();

    match tokio_tungstenite::connect_async(url).await {
        Ok((ws_socket, _)) => {
            let mut connection = ConnectionWs::new(ws_socket);
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