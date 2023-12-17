use std::sync::Arc;

use axum::{Form, response::{Redirect, IntoResponse}, debug_handler, extract::{State}, http::StatusCode, Json};
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{AppState, Account};

#[derive(Serialize,Deserialize)]
pub struct Payload {
    code: String,
}

#[derive(Serialize,Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
}

#[debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Form(payload): Form<Payload>,
)  -> Result<Json<TokenResponse>,StatusCode> {
    let hash = state.accounts.lock().await;

    match hash.get(&payload.code) {
        Some(_) => {
          Ok(Json(
              TokenResponse {
                access_token: payload.code,
                token_type: String::from("Bearer"),
              }
            ))
        },
        None => {
          Err(StatusCode::NOT_FOUND)
        },
    }
}
