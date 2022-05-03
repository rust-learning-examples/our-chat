use std::lazy::SyncOnceCell;
use std::collections::HashMap;
use tokio::sync::Mutex;
use crate::connection_ws;

static STATS_CELL: SyncOnceCell<Mutex<StateWs>>  = SyncOnceCell::new();

pub struct StateWs {
  pub writer_connections: HashMap<String, connection_ws::WriterConnectionWs<tokio::net::TcpStream>>
}

impl StateWs {
  pub fn global() -> &'static Mutex<StateWs> {
    STATS_CELL.get_or_init(|| {
      Mutex::new(StateWs {
        writer_connections: HashMap::new()
      })
    })
  }
  pub async fn insert_writer_connection(key: &str, value: connection_ws::WriterConnectionWs<tokio::net::TcpStream>) -> usize {
    let mut lock = StateWs::global().lock().await;
    (*lock).writer_connections.insert(key.to_string(), value);
    (*lock).writer_connections.len()
  }
  pub async fn remove_writer_connection(key: &str) -> usize {
    let mut lock = StateWs::global().lock().await;
    (*lock).writer_connections.remove(key);
    (*lock).writer_connections.len()
  }
}
