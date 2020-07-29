mod models;
mod oauth2client;
pub use models::{Options, Credentials};
pub use oauth2client::Oauth2client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
