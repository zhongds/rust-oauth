use reqwest;
use chrono::prelude::{Utc};
use serde_json::{Map, Value};
use serde::{Deserialize, Serialize};

pub struct Config {
  pub api_origin: String,
  pub client_id: String,
  pub client_secret: String,
  pub token_end_point: String,
  pub grant_type: String,
  pub user_agent: String,
  pub device_id: String,
  pub storage: String,
}

#[derive(Clone)]
pub struct Options {
  pub method: reqwest::Method,
  pub headers: reqwest::header::HeaderMap,
  pub is_with_credentials: bool,
  pub body: Option<Map<String, Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
  pub access_token: String,
  pub refresh_token: String,
  pub token_type: String,
  pub expires_in: i64,
  pub expires_at: Option<i64>,
  pub created_at: Option<i64>,
  pub user_id: String,
}

impl Credentials {
  pub fn init_expires_at(&mut self) {
    match self.expires_at {
      None => {
        self.expires_at = Some(Utc::now().timestamp() + self.expires_in - 30);
      },
      Some(_) => {},
    }
  }
  pub fn is_expired(&self) -> bool {
    if self.access_token == "" {
      return true;
    }
    match self.expires_at {
      None => true,
      Some(v) => {
        Utc::now().timestamp() > v
      },
    };
    return false;
  }
}
