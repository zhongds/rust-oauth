mod models;
mod oauth2client;
mod storage;
pub use models::{Options, Credentials, Config};
pub use oauth2client::{Oauth2client};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
