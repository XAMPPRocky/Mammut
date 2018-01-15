//! # Mammut: API Wrapper around the Mastodon API.
//!
//! Most of the api is documented on [Mastodon's
//! github](https://github.com/tootsuite/mastodon/blob/master/docs/Using-the-API/API.md#tag)
//!
//! ```no_run
//! # extern crate mammut;
//! # fn main() {
//! #    try().unwrap();
//! # }
//! # fn try() -> mammut::Result<()> {
//! use mammut::Registration;
//! use mammut::apps::{AppBuilder, Scopes};
//!
//! let app = AppBuilder {
//!     client_name: "mammut_test",
//!     redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
//!     scopes: Scopes::Read,
//!     website: None,
//! };
//!
//! let mut registration = Registration::new("https://mastodon.social");
//! registration.register(app)?;
//! let url = registration.authorise()?;
//! // Here you now need to open the url in the browser
//! // And handle a the redirect url coming back with the code.
//! let code = String::from("RETURNED_FROM_BROWSER");
//! let mastodon = registration.create_access_token(code)?;
//!
//! println!("{:?}", mastodon.get_home_timeline()?);
//! # Ok(())
//! # }
//! ```

#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, deny(missing_docs))]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json as json;
extern crate chrono;
extern crate reqwest;
extern crate serde;

/// Registering your App
pub mod apps;
/// Constructing a status
pub mod status_builder;
/// Entities returned from the API
pub mod entities;
/// Registering your app.
pub mod registration;

use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::ops;

use json::Error as SerdeError;
use reqwest::Error as HttpError;
use reqwest::{Client, StatusCode};
use reqwest::header::{Authorization, Bearer, Headers};

use entities::prelude::*;
pub use status_builder::StatusBuilder;

pub use registration::Registration;
/// Convience type over `std::result::Result` with `Error` as the error type.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! methods {
    ($($method:ident,)+) => {
        $(
            fn $method<T: for<'de> serde::Deserialize<'de>>(&self, url: String)
            -> Result<T>
            {
                use std::io::Read;

                let mut response = self.client.$method(&url)
                    .headers(self.headers.clone())
                    .send()?;

                let mut vec = Vec::new();
                response.read_to_end(&mut vec)?;

                if let Ok(t) = json::from_slice(&vec) {
                    Ok(t)
                } else {
                    Err(Error::Api(json::from_slice(&vec)?))
                }
            }
         )+
    };
}

macro_rules! route {

    ((post multipart ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        /// Equivalent to `/api/v1/
        #[doc = $url]
        /// `
        ///
        #[doc = "# Errors"]
        /// If `access_token` is not set.
        pub fn $name(&self, $($param: $typ,)*) -> Result<$ret> {
            use std::io::Read;
            use reqwest::multipart::Form;

            let form_data = Form::new()
            $(
                .file(stringify!($param), $param.as_ref())?
            )*;

            let mut response = self.client.post(&self.route(concat!("/api/v1/", $url)))
                .headers(self.headers.clone())
                .multipart(form_data)
                .send()?;

            let status = response.status().clone();

            if status.is_client_error() {
                return Err(Error::Client(status));
            } else if status.is_server_error() {
                return Err(Error::Server(status));
            }

            let mut vec = Vec::new();

            response.read_to_end(&mut vec)?;


            match json::from_slice::<$ret>(&vec) {
                Ok(res) => Ok(res),
                Err(_) => Err(Error::Api(json::from_slice(&vec)?)),
            }
        }

        route!{$($rest)*}
    };

    ((post ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        /// Equivalent to `/api/v1/
        #[doc = $url]
        /// `
        ///
        #[doc = "# Errors"]
        /// If `access_token` is not set.
        pub fn $name(&self, $($param: $typ,)*) -> Result<$ret> {
            use std::io::Read;

            let form_data = json!({
                $(
                    stringify!($param): $param,
                )*
            });

            let mut response = self.client.post(&self.route(concat!("/api/v1/", $url)))
                .headers(self.headers.clone())
                .json(&form_data)
                .send()?;

            let status = response.status().clone();

            if status.is_client_error() {
                return Err(Error::Client(status));
            } else if status.is_server_error() {
                return Err(Error::Server(status));
            }

            let mut vec = Vec::new();

            response.read_to_end(&mut vec)?;

            match json::from_slice(&vec) {
                Ok(res) => Ok(res),
                Err(_) => Err(Error::Api(json::from_slice(&vec)?)),
            }
        }

        route!{$($rest)*}
    };

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        /// Equivalent to `/api/v1/
        #[doc = $url]
        /// `
        ///
        #[doc = "# Errors"]
        /// If `access_token` is not set.
        pub fn $name(&self) -> Result<$ret> {
            self.$method(self.route(concat!("/api/v1/", $url)))
        }

        route!{$($rest)*}
    };

    () => {}
}

macro_rules! route_id {

    ($(($method:ident) $name:ident: $url:expr => $ret:ty,)*) => {
        $(
            /// Equivalent to `/api/v1/
            #[doc = $url]
            /// `
            ///
            #[doc = "# Errors"]
            /// If `access_token` is not set.
            pub fn $name(&self, id: u64) -> Result<$ret> {
                self.$method(self.route(&format!(concat!("/api/v1/", $url), id)))
            }
         )*
    }

}

/// Your mastodon application client, handles all requests to and from Mastodon.
#[derive(Clone, Debug)]
pub struct Mastodon {
    client: Client,
    headers: Headers,
    /// Raw data about your mastodon instance.
    pub data: Data
}

/// Raw data about mastodon app. Save `Data` using `serde` to prevent needing
/// to authenticate on every run.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Data {
    /// Base url of instance eg. `https://mastodon.social`.
    pub base: Cow<'static, str>,
    /// The client's id given by the instance.
    pub client_id: Cow<'static, str>,
    /// The client's secret given by the instance.
    pub client_secret: Cow<'static, str>,
    /// Url to redirect back to your application from the instance signup.
    pub redirect: Cow<'static, str>,
    /// The client's access token.
    pub token: Cow<'static, str>,
}

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
    Io(IoError),
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

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Api(ref e) => {
                e.error_description.as_ref().unwrap_or(&e.error)
            },
            Error::Serde(ref e) => e.description(),
            Error::Http(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
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
    pub error: String,
    /// The description of the error.
    pub error_description: Option<String>,
}

impl Mastodon {
    fn from_registration<I>(base: I,
                         client_id: I,
                         client_secret: I,
                         redirect: I,
                         token: I,
                         client: Client)
        -> Self
        where I: Into<Cow<'static, str>>
        {
            let data = Data {
                base: base.into(),
                client_id: client_id.into(),
                client_secret: client_secret.into(),
                redirect: redirect.into(),
                token: token.into(),

            };

            let mut headers = Headers::new();
            headers.set(Authorization(Bearer { token: (*data.token).to_owned() }));

            Mastodon {
                client: client,
                headers: headers,
                data: data,
            }
        }

    /// Creates a mastodon instance from the data struct.
    pub fn from_data(data: Data) -> Self {
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer { token: (*data.token).to_owned() }));

        Mastodon {
            client: Client::new(),
            headers: headers,
            data: data,
        }
    }

    route! {
        (get) verify: "accounts/verify_credentials" => Account,
        (get) blocks: "blocks" => Vec<Account>,
        (get) follow_requests: "follow_requests" => Vec<Account>,
        (get) mutes: "mutes" => Vec<Account>,
        (get) notifications: "notifications" => Vec<Notification>,
        (get) reports: "reports" => Vec<Report>,
        (get) get_home_timeline: "timelines/home" => Vec<Status>,
        (post (id: u64,)) allow_follow_request: "accounts/follow_requests/authorize" => Empty,
        (post (id: u64,)) reject_follow_request: "accounts/follow_requests/reject" => Empty,
        (post (uri: Cow<'static, str>,)) follows: "follows" => Account,
        (post) clear_notifications: "notifications/clear" => Empty,
        (post multipart (file: Cow<'static, str>,)) media: "media" => Attachment,
        (post (account_id: u64, status_ids: Vec<u64>, comment: String,)) report:
            "reports" => Report,
        (post (q: String, resolve: bool,)) search: "search" => SearchResult,
    }

    route_id! {
        (get) get_account: "accounts/{}" => Account,
        (get) followers: "accounts/{}/followers" => Vec<Account>,
        (get) following: "accounts/{}/following" => Vec<Account>,
        (get) follow: "accounts/{}/follow" => Account,
        (get) unfollow: "accounts/{}/unfollow" => Account,
        (get) block: "accounts/{}/block" => Account,
        (get) unblock: "accounts/{}/unblock" => Account,
        (get) mute: "accounts/{}/mute" => Account,
        (get) unmute: "accounts/{}/unmute" => Account,
        (get) get_notification: "notifications/{}" => Notification,
        (get) get_status: "statuses/{}" => Status,
        (get) get_context: "statuses/{}/context" => Context,
        (get) get_card: "statuses/{}/card" => Card,
        (get) reblogged_by: "statuses/{}/reblogged_by" => Vec<Account>,
        (get) favourited_by: "statuses/{}/favourited_by" => Vec<Account>,
        (post) reblog: "statuses/{}/reblog" => Status,
        (post) unreblog: "statuses/{}/unreblog" => Status,
        (post) favourite: "statuses/{}/favourite" => Status,
        (post) unfavourite: "statuses/{}/unfavourite" => Status,
        (delete) delete_status: "statuses/{}" => Empty,
    }

    /// Post a new status to the account.
    pub fn new_status(&self, status: StatusBuilder) -> Result<Status> {
        use std::io::Read;

        let mut response = self.client.post(&self.route("/api/v1/statuses"))
            .headers(self.headers.clone())
            .json(&status)
            .send()?;

        let mut vec = Vec::new();
        response.read_to_end(&mut vec)?;

        if let Ok(t) = json::from_slice(&vec) {
            Ok(t)
        } else {
            Err(Error::Api(json::from_slice(&vec)?))
        }
    }

    /// Get the federated timeline for the instance.
    pub fn get_public_timeline(&self, local: bool) -> Result<Vec<Status>> {
        let mut url = self.route("/api/v1/timelines/public");

        if local {
            url += "?local=1";
        }

        self.get(url)
    }

    /// Get timeline filtered by a hashtag(eg. `#coffee`) either locally or
    /// federated.
    pub fn get_tagged_timeline(&self, hashtag: String, local: bool) -> Result<Vec<Status>> {
        let mut url = self.route("/api/v1/timelines/tag/");
        url += &hashtag;

        if local {
            url += "?local=1";
        }

        self.get(url)
    }

    /// Get statuses of a single account by id. Optionally only with pictures
    /// and or excluding replies.
    pub fn statuses(&self, id: u64, only_media: bool, exclude_replies: bool)
        -> Result<Vec<Status>>
        {
            let mut url = format!("{}/api/v1/accounts/{}/statuses", self.base, id);

            if only_media {
                url += "?only_media=1";
            }

            if exclude_replies {
                url += if only_media {
                    "&"
                } else {
                    "?"
                };

                url += "exclude_replies=1";
            }

            self.get(url)
        }


    /// Returns the client account's relationship to a list of other accounts.
    /// Such as whether they follow them or vice versa.
    pub fn relationships(&self, ids: &[u64]) -> Result<Vec<Relationship>> {
        let mut url = self.route("/api/v1/accounts/relationships?");

        if ids.len() == 1 {
            url += "id=";
            url += &ids[0].to_string();
        } else {
            for id in ids {
                url += "id[]=";
                url += &id.to_string();
                url += "&";
            }
            url.pop();
        }

        self.get(url)
    }

    /// Search for accounts by their name.
    /// Will lookup an account remotely if the search term is in the
    /// `username@domain` format and not yet in the database.
    // TODO: Add a limit fn
    pub fn search_accounts(&self, query: &str) -> Result<Vec<Account>> {
        self.get(format!("{}/api/v1/accounts/search?q={}", self.base, query))
    }

    /// Returns the current Instance.
    pub fn instance(&self) -> Result<Instance> {
        self.get(self.route("/api/v1/instance"))
    }

    methods![get, post, delete,];

    fn route(&self, url: &str) -> String {
        let mut s = (*self.base).to_owned();
        s += url;
        s
    }
}

impl ops::Deref for Mastodon {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

macro_rules! from {
    ($($typ:ident, $variant:ident,)*) => {
        $(
            impl From<$typ> for Error {
                fn from(from: $typ) -> Self {
                    use Error::*;
                    $variant(from)
                }
            }
        )*
    }
}

from! {
    SerdeError, Serde,
    HttpError, Http,
    IoError, Io,
}
