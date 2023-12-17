use std::{collections::HashMap, sync::Arc};

mod config;

use axum::{
  routing::{get, post},
  Router,
};
use axum_sessions::{SessionLayer, async_session::MemoryStore};
use tokio::sync::Mutex;

mod api;

#[derive(PartialEq, Eq, Hash, Clone)] // Implement Clone trait
struct Account{
    name: String,
    mail: String,
}

type Accounts = HashMap<String, Account>;

#[derive(Clone)]
pub struct AppState {
    accounts: Arc<Mutex<Accounts>>,
}

#[tokio::main]
async fn main() {
  let accounts: Accounts = HashMap::new();

  let state: AppState = AppState {
    accounts: Arc::new(Mutex::new(accounts)),
  };

  let store = MemoryStore::new();
  let secret = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; // MUST be at least 64 bytes!
  let session_layer = SessionLayer::new(store, secret);

  let config = config::Config::new();

  let app = Router::new()
                        .route(&config.get_login_path, get(api::login_get::execute))
                        .route(&config.post_login_path, post(api::login_post::execute))
                        .route(&config.get_info_path, get(api::info_get::execute))
                        .route(&config.post_token_path, post(api::token_post::execute))
                        .with_state(state)
                        .layer(session_layer);

  // run it with hyper on localhost:3000

  let socket_addr = format!("{}:{}", config.address, config.port);

  axum::Server::bind(&socket_addr.parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}
