use tokio::net::{TcpListener, TcpStream};
use wetalk::{StateWs};
use std::{env, net::{SocketAddr}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // pretty_env_logger::init();

    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
    // with addr, eg: RUST_LOG=debug cargo run -p ws_server -- 127.0.0.1:12345
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:5555".to_string());
    // 监听指定地址，等待 TCP 连接进来
    log::info!("Listen on: {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        // 第二个被忽略的项中包含有新连接的 `IP` 和端口信息
        let (socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let addr_str = addr.to_string();
            if process(socket, addr).await.is_err() {
                let len = StateWs::remove_writer_connection(&addr_str).await;
                log::info!("disconnect: {:?}, connected size: {}", addr, len);
            }
        });
    }
}

async fn process(socket: TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
    let addr_str = addr.to_string();
    let ws_socket = wetalk::tokio_tungstenite::accept_async(socket).await?;
    let (mut reader, writer) =  wetalk::connection_ws::ConnectionWs::new(ws_socket).split();
    let len = StateWs::insert_writer_connection(&addr_str, writer).await;
    log::info!("connect: {}, connected size: {}", addr_str, len);

    loop {
        match reader.read_message().await {
            Ok(message) => {
                match message {
                    wetalk::Message::Text(message) => {
                        log::debug!("recv text message: {}", message);
                        let mut lock = StateWs::global().lock().await;
                        if let Some(writer) = (*lock).writer_connections.get_mut(&addr_str) {
                            // writer.write_message(wetalk::Message::Text(message.clone())).await?;
                            writer.write_text(&message).await?;
                            log::debug!("send back message: {}", message);
                        }
                    },
                    wetalk::Message::Close(close_frame) => {
                        log::debug!("client disconnected, err: {:?}", close_frame);
                        return Err(anyhow::anyhow!("client disconnected"))
                    },
                    _ => ()
                }
            },
            Err(err) => {
                log::debug!("client disconnected, err: {:?}", err);
                return Err(err)
            }
        }
    }
    // Ok(())
}
