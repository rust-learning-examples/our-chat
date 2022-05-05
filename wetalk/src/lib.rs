#![feature(once_cell)]

// mod types;
#[cfg(feature = "tcp")]
pub mod state;
#[cfg(feature = "ws_tungstenite")]
pub mod state_ws;
#[cfg(feature = "ws_axum")]
pub mod state_axum;
#[cfg(feature = "tcp")]
pub mod connection;
#[cfg(feature = "ws_tungstenite")]
pub mod connection_ws;
#[cfg(feature = "ws_axum")]
pub mod connection_axum;

#[cfg(feature = "tcp")]
pub use state::State;
#[cfg(feature = "ws_tungstenite")]
pub use state_ws::StateWs;
#[cfg(feature = "ws_axum")]
pub use state_axum::StateAxum;
#[cfg(feature = "tcp")]
pub use connection::Connection;
#[cfg(feature = "ws_tungstenite")]
pub use connection_ws::ConnectionWs;
#[cfg(feature = "ws_axum")]
pub use connection_axum::ConnectionAxum;
#[cfg(any(feature = "tcp", feature = "ws_tungstenite"))]
pub use message::TSMessage;
#[cfg(feature = "ws_axum")]
pub use message::AXMessage;

#[cfg(feature = "ws_tungstenite")]
pub use tokio_tungstenite;


pub mod message;
pub use anyhow;