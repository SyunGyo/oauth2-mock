use axum::{Form, debug_handler, extract::State, http::StatusCode, Json};
use serde::{Serialize, Deserialize};

use crate::AppState;

#[derive(Serialize,Deserialize)]
pub struct Payload {
    access_token: String,
}

#[derive(Serialize,Deserialize)]
pub struct InfoResponse {
    name: String,
    email: String,
}

#[debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    Form(payload): Form<Payload>,
)  -> Result<Json<InfoResponse>,StatusCode> {
    let hash = state.accounts.lock().await;

    match hash.get(&payload.access_token) {
      Some(account) => {
          Ok(Json(
              InfoResponse {
                name: account.name.clone(),
                email: account.mail.clone(),
              }
            ))
      },
      None => {
          Err(StatusCode::UNAUTHORIZED)
      },
    }
}
