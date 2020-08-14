use reqwest;
use std::string::String;
use serde::de::DeserializeOwned;
use serde_json::{Value, Map};
// use chrono::prelude::{DateTime, Utc};
use std::error::Error;

use crate::models::{Config, Credentials, Options};
use crate::storage::Storage;

pub struct Oauth2client {
  pub config: Config,
}

// pub trait Oauth2clientTrait {
//   fn get_access_token(&self) -> String;
//   fn set_credentials(&self, credentials: Credentials);
//   fn has_credentials(&self) -> bool;
//   fn request(&self, url: String, opts: Options) -> reqwest::Result<String>;
// }

// static GLOBAL: state::Storage<u32> = Storage::new();
// static COUNT: LocalStorage<Cell<usize>> = LocalStorage::new();

const CREDENTIALS_KEY: &str = "credentials_key";

impl Oauth2client {
  pub fn set_credentials(&self, credentials: &Credentials) {
    let credentials_str = serde_json::to_string(credentials).unwrap_or("".to_string());
    Storage::set_item(CREDENTIALS_KEY, &credentials_str);
  }

  fn has_credentials(&self) -> bool {
    let item = Storage::get_item(CREDENTIALS_KEY);
    return item != "";
  }

  // 同步 -> 异步
  #[tokio::main]
  pub async fn request<T: DeserializeOwned>(&self, url: String, opts: Options) -> reqwest::Result<T> {
    // 获取token
    let mut headers = opts.headers;
    if opts.is_with_credentials {
      let token: String = self.get_access_token().unwrap_or("".to_string());
      if token == "" {
        panic!("token  is not found");
      }
      let mut auth_token = String::from("Bearer ");
      auth_token.push_str(&token);
      headers.insert("Authorization", auth_token.parse().unwrap());
    }

    let body = opts.body.unwrap_or(Map::new());

    let client = reqwest::Client::new();
    let res = client.request(opts.method, &url)
      .headers(headers)
      .json(&body)
      .send()
      .await?
      .json::<T>()
      .await?;

    Ok(res)
  }

  pub fn get_access_token(&self) -> Result<String, Box<dyn Error>> {
    let credentials_str = Storage::get_item(CREDENTIALS_KEY);
    if credentials_str == "" {
      panic!("credentials is not found");
    }
    let credentials: Credentials = serde_json::from_str(&credentials_str).unwrap();
    let is_expired = credentials.is_expired();
    if is_expired {
      self.refresh_token(credentials.refresh_token);
    }
    Ok(credentials.access_token)
  }

  pub fn refresh_token(&self, refresh_token: String) {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let mut map = Map::new();
    map.insert("client_id".to_string(), Value::String(self.config.client_id.to_string()));
    map.insert("grant_type".to_string(), Value::String("refresh_token".to_string()));
    map.insert("refresh_token".to_string(), Value::String(refresh_token));
    let params = Options {
      method: reqwest::Method::POST,
      headers: headers,
      is_with_credentials: false,
      body: Some(map),
    };
    let credentials: Credentials = self.request("/v1/auth/token".to_string(), params).unwrap();
    self.set_credentials(&credentials);
  }
}

#[cfg(test)]
mod tests {
  use super::{Oauth2client, Config, Options};
  use serde::{Deserialize};
  use reqwest;

  #[test]
  fn test_get_access_token() {
    let conf = Config{
      api_origin: String::from("api_origin"),
      client_id: String::from("clientid"),
      client_secret: String::from("client_secret"),
      token_end_point: String::from("token_end_point"),
      grant_type: String::from("grant_type"),
      user_agent: String::from("user_agent"),
      device_id: String::from("device_id"),
      storage: String::from("storage"),
    };
    // let credentials: Credentials = ();
    let client = Oauth2client{config: conf};
    let tokenstr = client.get_access_token();
    assert_eq!(tokenstr, "api_origin");
  }

  #[test]
  fn test_request() {
    
    let conf = Config{
      api_origin: String::from("api_origin"),
      client_id: String::from("clientid"),
      client_secret: String::from("client_secret"),
      token_end_point: String::from("token_end_point"),
      grant_type: String::from("grant_type"),
      user_agent: String::from("user_agent"),
      device_id: String::from("device_id"),
      storage: String::from("storage"),
    };
    let client = Oauth2client{config: conf};
    let opts = Options{
      method: reqwest::Method::GET,
      headers: reqwest::header::HeaderMap::new(),
      is_with_credentials: false,
      body: None,
    };

    #[derive(Deserialize)]
    struct TName {
      name: String,
    }
    #[derive(Deserialize)]
    struct TAge {
      age: u32,
    }

    let url = "https://www.fastmock.site/mock/6bb4e8c4fd256ca2fd96696c7eb7cf48/rust_demo/getname";
    let tname: TName = client.request(String::from(url), opts).unwrap();
    assert_eq!(tname.name, "zds");

    let opts2 = Options {
      method: reqwest::Method::GET,
      headers: reqwest::header::HeaderMap::new(),
      is_with_credentials: false,
      body: None,
    };
    let url = "https://www.fastmock.site/mock/6bb4e8c4fd256ca2fd96696c7eb7cf48/rust_demo/getage";
    let tage: TAge = client.request(String::from(url), opts2).unwrap();

    assert_eq!(tage.age, 18);
  }

  #[test]
  fn test_signin() {
    assert_eq!(1+1,2);
  }
}

