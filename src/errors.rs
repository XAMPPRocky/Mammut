use std::{
    io,
    fmt,
    error
};

use json::Error as SerdeError;
use reqwest::Error as HttpError;
use reqwest::StatusCode;
use url::ParseError as UrlError;

/// Convience type over `std::result::Result` with `Error` as the error type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// enum of possible errors encountered using the mastodon API.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Error {
    /// Error from the Mastodon API. This typically means something went
    /// wrong with your authentication or data.
    Api(ApiError),
    /// Error deserialising to json. Typically represents a breaking change in
    /// the Mastodon API
    #[serde(skip_deserializing)]
    Serde(SerdeError),
    /// Error encountered in the HTTP backend while requesting a route.
    #[serde(skip_deserializing)]
    Http(HttpError),
    /// Wrapper around the `std::io::Error` struct.
    #[serde(skip_deserializing)]
    Io(io::Error),
    /// Wrapper around the `url::ParseError` struct.
    #[serde(skip_deserializing)]
    Url(UrlError),
    /// Missing Client Id.
    #[serde(skip_deserializing)]
    ClientIdRequired,
    /// Missing Client Secret.
    #[serde(skip_deserializing)]
    ClientSecretRequired,
    /// Missing Access Token.
    #[serde(skip_deserializing)]
    AccessTokenRequired,
    /// Generic client error.
    #[serde(skip_deserializing)]
    Client(StatusCode),
    /// Generic server error.
    #[serde(skip_deserializing)]
    Server(StatusCode),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Api(ref e) => {
                e.error_description.as_ref().map(|i| &**i)
                    .or(e.error.as_ref().map(|i| &**i))
                    .unwrap_or("Unknown API Error")
            },
            Error::Serde(ref e) => e.description(),
            Error::Http(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::Url(ref e) => e.description(),
            Error::Client(ref status) | Error::Server(ref status) => {
                status.canonical_reason().unwrap_or("Unknown Status code")
            },
            Error::ClientIdRequired => "ClientIdRequired",
            Error::ClientSecretRequired => "ClientSecretRequired",
            Error::AccessTokenRequired => "AccessTokenRequired",
        }
    }
}

/// Error returned from the Mastodon API.
#[derive(Clone, Debug, Deserialize)]
pub struct ApiError {
    /// The type of error.
    pub error: Option<String>,
    /// The description of the error.
    pub error_description: Option<String>,
}

