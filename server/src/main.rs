use tokio::net::{TcpListener, TcpStream};
use wetalk::Connection;
use snafu::{Whatever};

#[tokio::main]
async fn main() -> Result<(), Whatever> {
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:5555").await.unwrap();

    loop {
        // 第二个被忽略的项中包含有新连接的 `IP` 和端口信息
        let (socket, addr) = listener.accept().await.unwrap();
        println!("connect: {:?}", addr);
        if process(socket).await.is_err() {
            println!("connect close: {:?}", addr);
        }
    }
}

async fn process(socket: TcpStream) -> Result<(), Whatever> {
    let mut connection = Connection::new(socket);
    loop {
        let buf = connection.read_data().await?;
        let text = String::from_utf8_lossy(&buf[..]);
        println!("get: {}", text);
        if connection.write_text(text.into()).await.is_err() {
            // 非预期错误，由于我们这里无需再做什么，因此直接停止处理
            break
        }
    }
    Ok(())
}
