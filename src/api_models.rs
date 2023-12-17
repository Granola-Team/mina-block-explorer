use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MyError {
    NetworkError(String),
    ParseError(String), // other error variants
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> Self {
        MyError::NetworkError(err.to_string())
    }
}
