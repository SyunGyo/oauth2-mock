use std::arch::x86_64::_SIDD_CMP_EQUAL_ORDERED;
use std::fs::File;
use std::io::Read;

use axum::{
  extract::Query,
  response::{Html, IntoResponse, Response, ErrorResponse},
  http::{StatusCode},
};
use axum_sessions::extractors::WritableSession;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginQuery {
  redirect_uri: String,
  state: String,
}

pub async fn execute(
  Query(query): Query<LoginQuery>,
  mut session: WritableSession,
) -> Result<Html<String>, StatusCode> {
  let redirect_uri = query.redirect_uri;
  let state = query.state;
  match session.insert("redirect_uri", &redirect_uri) {
    Ok(_) => (),
    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
  };

  match session.insert("state", &state) {
    Ok(_) => (),
    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
  };

  Ok(Html(
    template()
  ))
}

fn template() -> String {
    let mut f = File::open("src/login_page.html").expect("file not found");

    let mut login_page = String::new();
    f.read_to_string(&mut login_page)
      .expect("something went wrong reading the file");

  return login_page;
}
