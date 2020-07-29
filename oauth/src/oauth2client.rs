use reqwest;
use std::string::String;

use crate::models::{Options, Credentials};

#[derive(Clone)]
pub struct Oauth2client {
  options: Options,
}

pub trait Oauth2clientTrait {
  fn get_access_token(&self) -> String;
  fn set_credentials(&self, credentials: Credentials);
  fn has_credentials(&self) -> bool;
  fn request(&self, url: String) -> reqwest::Result<String>;
}

impl Oauth2clientTrait for Oauth2client {
  fn get_access_token(&self) -> String {
    return "access token".to_string();
  }

  fn set_credentials(&self, credentials: Credentials) {
    println!("set credentials: {}", credentials.user_id);
  }

  fn has_credentials(&self) -> bool {
    return true;
  }

  #[tokio::main]
  async fn request(&self, _url: String) -> reqwest::Result<String> {
    let res = reqwest::get("https://hyper.rs").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(body)
  }
}

#[cfg(test)]
mod tests {
  use super::{Oauth2client, Options, Oauth2clientTrait};

  #[test]
  fn test_get_access_token() {
    let opts = Options{
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
    let client = Oauth2client{options: opts};
    let tokenstr = client.get_access_token();
    assert_eq!(tokenstr, "access token");
  }
}

