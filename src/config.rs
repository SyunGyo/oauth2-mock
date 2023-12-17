use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
  pub port: u8,
  pub address: String,
  pub get_login_path: String,
  pub post_login_path: String,
  pub post_token_path: String,
  pub get_info_path: String,
}

impl Config {
  pub fn new() -> Self {
    let settings = config::Config::builder()
      .add_source(config::File::with_name("Config.yaml"))
      .build()
      .unwrap();

    settings.try_deserialize().unwrap()
  }
}
