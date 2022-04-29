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
        let buf = connection.read_data().await?;
        let text = String::from_utf8_lossy(&buf[..]);
        log::debug!("get text: {:?}", text);

        match connection.write_text(text.into()).await {
            Ok(_) => (),
            Err(e) => {
                // 非预期错误，比如连接中断
                log::debug!("err: {}", e);
                return Err(e)
            }
        }
    }
    // Ok(())
}
