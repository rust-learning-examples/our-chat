#[cfg(any(feature = "tcp", feature = "ws_tungstenite"))]
pub use tokio_tungstenite::tungstenite::Message as TSMessage;
#[cfg(any(feature = "tcp", feature = "ws_tungstenite"))]
pub use tokio_tungstenite::tungstenite::protocol::frame::{CloseFrame as TSCloseFrame, coding::CloseCode as TSCloseCode};
#[cfg(feature = "ws_axum")]
pub use axum::extract::ws::{Message as AXMessage};

// pub trait MessageAble {}
// impl MessageAble for TSMessage {}
// impl MessageAble for AXMessage {}