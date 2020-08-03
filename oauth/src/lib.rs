mod models;
mod oauth2client;
pub use models::{Options, Credentials, Config};
pub use oauth2client::{Oauth2client, Oauth2clientTrait};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
