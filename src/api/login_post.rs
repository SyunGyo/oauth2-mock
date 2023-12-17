use axum::{Form, response::Redirect, debug_handler, extract::State, http::StatusCode};
use serde::{Serialize, Deserialize};
use axum_sessions::extractors::ReadableSession;
use crate::{AppState, Account};


use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct Payload {
    name: String,
    mail: String,
}

#[debug_handler]
pub async fn execute(
    State(state): State<AppState>,
    session: ReadableSession,
    Form(payload): Form<Payload>,
)  -> Result<Redirect,StatusCode> {

    let code = Uuid::new_v4().to_string();
    state.accounts.lock().await.insert(code.clone(), Account {
        name: payload.name,
        mail: payload.mail,
    });
    let mut redirect_uri = match session.get::<String>("redirect_uri") {
        Some(redirect_uri) => redirect_uri,
        None => {
          return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    let state = session.get::<String>("state").unwrap_or("".to_string());

    redirect_uri.push_str(format!("?code={}&state={}", code, state).as_str());
    Ok(Redirect::to(redirect_uri.as_str()))
}


