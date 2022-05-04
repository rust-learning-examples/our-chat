use std::lazy::SyncOnceCell;
use std::collections::HashMap;
use tokio::sync::Mutex;
use crate::connection_axum;

static STATS_CELL: SyncOnceCell<Mutex<StateAxum>>  = SyncOnceCell::new();

pub struct StateAxum {
  pub writer_connections: HashMap<String, connection_axum::WriterConnectionAxum>
}

impl StateAxum {
  pub fn global() -> &'static Mutex<StateAxum> {
    STATS_CELL.get_or_init(|| {
      Mutex::new(StateAxum {
        writer_connections: HashMap::new()
      })
    })
  }
  pub async fn insert_writer_connection(key: &str, value: connection_axum::WriterConnectionAxum) -> usize {
    let mut lock = StateAxum::global().lock().await;
    (*lock).writer_connections.insert(key.to_string(), value);
    (*lock).writer_connections.len()
  }
  pub async fn remove_writer_connection(key: &str) -> usize {
    let mut lock = StateAxum::global().lock().await;
    (*lock).writer_connections.remove(key);
    (*lock).writer_connections.len()
  }
}
