use reqwest;
use std::string::String;

use crate::models::{Config, Credentials, Options};

pub struct Oauth2client {
  pub config: Config,
}

pub trait Oauth2clientTrait {
  fn get_access_token(&self) -> String;
  fn set_credentials(&self, credentials: Credentials);
  fn has_credentials(&self) -> bool;
  fn request(&self, url: String, opts: Options) -> reqwest::Result<String>;
}

impl Oauth2clientTrait for Oauth2client {
  fn get_access_token(&self) -> String {
    return self.config.api_origin.to_string();
  }

  fn set_credentials(&self, credentials: Credentials) {
    println!("set credentials: {}", credentials.user_id);
  }

  fn has_credentials(&self) -> bool {
    return true;
  }

  #[tokio::main]
  async fn request(&self, url: String, opts: Options) -> reqwest::Result<String>{

    let client = reqwest::Client::new();
    // let res = client.post("http://httpbin.org/post")
    // .body("the exact body that is sent")
    // .send()
    // .await?
    // .text()
    // .await?;
    let res = client.request(opts.method, &url)
              .headers(opts.headers)
              .send()
              .await?
              .text()
              .await?;
    Ok(res)
  }
}

#[cfg(test)]
mod tests {
  use super::{Oauth2client, Config, Oauth2clientTrait, Options};

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
    };
    let token = client.request(String::from("http://localhost:7878/"), opts);
    println!("token: {:#?}", token);
    assert_eq!(1+1, 2);
  }
}

