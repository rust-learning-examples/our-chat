use std::{env, net::{SocketAddr}};
use wetalk::{StateAxum};
use axum::extract::ws;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // pretty_env_logger::init();
    tracing_subscriber::fmt::init();
    // with addr, eg: RUST_LOG=debug cargo run -p tcp_server -- 127.0.0.1:12345
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:5555".to_string());
    // 监听指定地址，等待 TCP 连接进来
    log::info!("Listen on: {}", addr);


    let app = axum::Router::new()
        .route("/", axum::routing::get(websocket_handler))
        // logging so we can see whats going on
        .layer(tower_http::trace::TraceLayer::new_for_http().make_span_with(tower_http::trace::DefaultMakeSpan::default().include_headers(true)));

    // run it with hyper
    let addr: SocketAddr = addr.parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

    Ok(())

}

async fn websocket_handler(
    ws: axum::extract::ws::WebSocketUpgrade,
    req: axum::http::Request<axum::body::Body>
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(|socket| async {
        let ident = "first".to_string();
        if handle_socket(socket, req, ident.clone()).await.is_err() {
            let len = StateAxum::remove_writer_connection(&ident).await;
                log::info!("disconnect: {:?}, connected size: {}", ident, len);
        }
    })
}

async fn handle_socket(socket: ws::WebSocket, _req: axum::http::Request<axum::body::Body>, ident: String) -> wetalk::anyhow::Result<()> {
    let (mut reader, writer) =  wetalk::connection_axum::ConnectionAxum::new(socket).split();
    let len = StateAxum::insert_writer_connection(&ident, writer).await;
    log::info!("connect: {}, connected size: {}", ident, len);

    loop {
        match reader.read_message().await {
            Ok(message) => {
                match message {
                    ws::Message::Text(message) => {
                        log::debug!("recv text message: {}", message);
                        let mut lock = StateAxum::global().lock().await;
                        if let Some(writer) = (*lock).writer_connections.get_mut(&ident) {
                            // writer.write_message(wetalk::Message::Text(message.clone())).await?;
                            writer.write_text(&message).await?;
                            log::debug!("send back message: {}", message);
                        }
                    },
                    ws::Message::Close(close_frame) => {
                        log::debug!("client disconnected, err: {:?}", close_frame);
                        return Err(anyhow::anyhow!("client disconnected"))
                    },
                    _ => ()
                }
            },
            Err(err) => {
                log::debug!("client disconnected2, err: {:?}", err);
                return Err(err)
            }
        }
    }
}
