use std::lazy::SyncOnceCell;
use std::collections::HashMap;
use tokio::sync::Mutex;
use crate::connection;

static STATS_CELL: SyncOnceCell<Mutex<State>>  = SyncOnceCell::new();

pub struct State {
  pub writer_connections: HashMap<String, connection::WriterConnection>
}

impl State {
  pub fn global() -> &'static Mutex<State> {
    STATS_CELL.get_or_init(|| {
      Mutex::new(State {
        writer_connections: HashMap::new()
      })
    })
  }
  pub async fn insert_writer_connection(key: &str, value: connection::WriterConnection) -> usize {
    let mut lock = State::global().lock().await;
    (*lock).writer_connections.insert(key.to_string(), value);
    (*lock).writer_connections.len()
  }
  pub async fn remove_writer_connection(key: &str) -> usize {
    let mut lock = State::global().lock().await;
    (*lock).writer_connections.remove(key);
    (*lock).writer_connections.len()
  }
}
