use tokio::net::{TcpListener, TcpStream};
use wetalk::Connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    // 监听指定地址，等待 TCP 连接进来
    log::info!("listen on 127.0.0.1:5555");
    let listener = TcpListener::bind("127.0.0.1:5555").await.unwrap();

    loop {
        // 第二个被忽略的项中包含有新连接的 `IP` 和端口信息
        let (socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            log::info!("connect: {:?}", addr);
            if process(socket).await.is_err() {
                log::info!("disconnect: {:?}", addr);
            }
        });
    }
}

async fn process(socket: TcpStream) -> anyhow::Result<()> {
    let mut connection = Connection::new(socket);
    loop {
        match connection.read_message().await {
            Ok(message) => {
                match message {
                    wetalk::Message::Text(message) => {
                        connection.write_message(wetalk::Message::Text(message.clone())).await?;
                        log::debug!("recv text message: {}, and send back", message);
                    },
                    wetalk::Message::Close(err) => {
                        log::debug!("client disconnected, err: {:?}", err);
                        return Err(err)
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
