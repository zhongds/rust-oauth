#[derive(Clone)]
pub struct Options {
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
pub struct Credentials {
  pub access_token: String,
  pub refresh_token: String,
  pub token_type: String,
  pub expires_in: String,
  pub expires_at: String,
  pub created_at: String,
  pub user_id: String,
}
