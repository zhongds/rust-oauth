// use oauth::{Oauth2client, Config, Oauth2clientTrait};

// #[test]
// fn test_oauth2client_get_token() {
//   let conf = Config{
//     api_origin: String::from("api_origin1"),
//     client_id: String::from("clientid"),
//     client_secret: String::from("client_secret"),
//     token_end_point: String::from("token_end_point"),
//     grant_type: String::from("grant_type"),
//     user_agent: String::from("user_agent"),
//     device_id: String::from("device_id"),
//     storage: String::from("storage"),
//   };

//   // println!("config: {:#?}", conf);
//   // let credentials: Credentials = ();
//   let client = Oauth2client{config: conf};
//   let tokenstr = client.get_access_token();
//   assert_eq!(tokenstr, "api_origin1");
// }

#[test]
fn it_works_111() {
  assert_eq!(2 + 2, 4);
}
