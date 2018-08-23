//! # Mammut: API Wrapper around the Mastodon API.
//!
//! Most of the api is documented on [Mastodon's
//! github](https://github.com/tootsuite/documentation/blob/master/Using-the-API/API.md)
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
//! println!("{:?}", mastodon.get_home_timeline()?.initial_items);
//! # Ok(())
//! # }
//! ```

#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, deny(missing_docs))]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate doc_comment;
#[macro_use] extern crate serde_json as json;
extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate url;

/// Registering your App
pub mod apps;
/// Constructing a status
pub mod status_builder;
/// Entities returned from the API
pub mod entities;
/// Registering your app.
pub mod registration;
/// Handling multiple pages of entities.
pub mod page;

use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::ops;

use json::Error as SerdeError;
use reqwest::Error as HttpError;
use reqwest::{Client, Response, StatusCode};
use reqwest::header::{Authorization, Bearer, Headers};
use url::ParseError as UrlError;

use entities::prelude::*;
pub use status_builder::StatusBuilder;
use page::Page;

pub use registration::Registration;
/// Convience type over `std::result::Result` with `Error` as the error type.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! methods {
    ($($method:ident,)+) => {
        $(
            fn $method<T: for<'de> serde::Deserialize<'de>>(&self, url: String)
            -> Result<T>
            {
                let response = self.client.$method(&url)
                    .headers(self.headers.clone())
                    .send()?;

                deserialise(response)
            }
         )+
    };
}

macro_rules! paged_routes {

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `/api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),
            pub fn $name(&self) -> Result<Page<$ret>> {
                let url = self.route(concat!("/api/v1/", $url));
                let response = self.client.$method(&url)
                    .headers(self.headers.clone())
                    .send()?;

                Page::new(self, response)
            }

        }

        paged_routes!{$($rest)*}
    };

    () => {}
}

macro_rules! route {

    ((post multipart ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `/api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),
            pub fn $name(&self, $($param: $typ,)*) -> Result<$ret> {
                use reqwest::multipart::Form;

                let form_data = Form::new()
                    $(
                        .file(stringify!($param), $param.as_ref())?
                     )*;

                let response = self.client.post(&self.route(concat!("/api/v1/", $url)))
                    .headers(self.headers.clone())
                    .multipart(form_data)
                    .send()?;

                let status = response.status().clone();

                if status.is_client_error() {
                    return Err(Error::Client(status));
                } else if status.is_server_error() {
                    return Err(Error::Server(status));
                }

                deserialise(response)
            }
        }

        route!{$($rest)*}
    };

    (($method:ident ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `/api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),

            pub fn $name(&self, $($param: $typ,)*) -> Result<$ret> {

                let form_data = json!({
                    $(
                        stringify!($param): $param,
                    )*
                });

                let response = self.client.$method(&self.route(concat!("/api/v1/", $url)))
                    .headers(self.headers.clone())
                    .json(&form_data)
                    .send()?;

                let status = response.status().clone();

                if status.is_client_error() {
                    return Err(Error::Client(status));
                } else if status.is_server_error() {
                    return Err(Error::Server(status));
                }

                deserialise(response)
            }
        }

        route!{$($rest)*}
    };

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `/api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),
            pub fn $name(&self) -> Result<$ret> {
                self.$method(self.route(concat!("/api/v1/", $url)))
            }
        }

        route!{$($rest)*}
    };

    () => {}
}

macro_rules! route_id {

    ($(($method:ident) $name:ident: $url:expr => $ret:ty,)*) => {
        $(
            doc_comment! {
                concat!(
                    "Equivalent to `/api/v1/",
                    $url,
                    "`\n# Errors\nIf `access_token` is not set."),
                pub fn $name(&self, id: u64) -> Result<$ret> {
                    self.$method(self.route(&format!(concat!("/api/v1/", $url), id)))
                }
            }
         )*
    }

}
macro_rules! paged_routes_with_id {

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `/api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),
            pub fn $name(&self, id: &str) -> Result<Page<$ret>> {
                let url = self.route(&format!(concat!("/api/v1/", $url), id));
                let response = self.client.$method(&url)
                    .headers(self.headers.clone())
                    .send()?;

                Page::new(self, response)
            }
        }

        route!{$($rest)*}
    };
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

impl StdError for Error {
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

/// # Example
///
/// ```
/// # extern crate mammut;
/// # use mammut::StatusesRequest;
/// let request = StatusesRequest::new()
///                               .only_media()
///                               .pinned()
///                               .since_id("foo");
/// # assert_eq!(&request.to_querystring()[..], "?only_media=1&pinned=1&since_id=foo");
/// ```
#[derive(Clone, Debug, Default)]
pub struct StatusesRequest<'a> {
    only_media: bool,
    exclude_replies: bool,
    pinned: bool,
    max_id: Option<Cow<'a, str>>,
    since_id: Option<Cow<'a, str>>,
    limit: Option<usize>,
}

impl<'a> StatusesRequest<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn only_media(mut self) -> Self {
        self.only_media = true;
        self
    }

    pub fn exclude_replies(mut self) -> Self {
        self.exclude_replies = true;
        self
    }

    pub fn pinned(mut self) -> Self {
        self.pinned = true;
        self
    }

    pub fn max_id<S: Into<Cow<'a, str>>>(mut self, max_id: S) -> Self {
        self.max_id = Some(max_id.into());
        self
    }

    pub fn since_id<S: Into<Cow<'a, str>>>(mut self, since_id: S) -> Self {
        self.since_id = Some(since_id.into());
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn to_querystring(&self) -> String {
        let mut opts = vec![];

        if self.only_media {
            opts.push("only_media=1".into());
        }

        if self.exclude_replies {
            opts.push("exclude_replies=1".into());
        }

        if self.pinned {
            opts.push("pinned=1".into());
        }

        if let Some(ref max_id) = self.max_id {
            opts.push(format!("max_id={}", max_id));
        }

        if let Some(ref since_id) = self.since_id {
            opts.push(format!("since_id={}", since_id));
        }

        if let Some(limit) = self.limit {
            opts.push(format!("limit={}", limit));
        }

        if opts.is_empty() {
            String::new()
        } else {
            format!("?{}", opts.join("&"))
        }
    }
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

    paged_routes! {
        (get) favourites: "favourites" => Status,
        (get) blocks: "blocks" => Account,
        (get) domain_blocks: "domain_blocks" => String,
        (get) follow_requests: "follow_requests" => Account,
        (get) get_home_timeline: "timelines/home" => Status,
        (get) get_emojis: "custom_emojis" => Emoji,
        (get) mutes: "mutes" => Account,
        (get) notifications: "notifications" => Notification,
        (get) reports: "reports" => Report,
    }

    paged_routes_with_id! {
        (get) followers: "accounts/{}/followers" => Account,
        (get) following: "accounts/{}/following" => Account,
        (get) reblogged_by: "statuses/{}/reblogged_by" => Account,
        (get) favourited_by: "statuses/{}/favourited_by" => Account,
    }

    route! {
        (delete (domain: String,)) unblock_domain: "domain_blocks" => Empty,
        (get) instance: "instance" => Instance,
        (get) verify_credentials: "accounts/verify_credentials" => Account,
        (post (account_id: &str, status_ids: Vec<&str>, comment: String,)) report: "reports" => Report,
        (post (domain: String,)) block_domain: "domain_blocks" => Empty,
        (post (id: &str,)) authorize_follow_request: "accounts/follow_requests/authorize" => Empty,
        (post (id: &str,)) reject_follow_request: "accounts/follow_requests/reject" => Empty,
        (post (q: String, resolve: bool,)) search: "search" => SearchResult,
        (post (uri: Cow<'static, str>,)) follows: "follows" => Account,
        (post multipart (file: Cow<'static, str>,)) media: "media" => Attachment,
        (post) clear_notifications: "notifications/clear" => Empty,
    }

    route_id! {
        (get) get_account: "accounts/{}" => Account,
        (post) follow: "accounts/{}/follow" => Account,
        (post) unfollow: "accounts/{}/unfollow" => Account,
        (get) block: "accounts/{}/block" => Account,
        (get) unblock: "accounts/{}/unblock" => Account,
        (get) mute: "accounts/{}/mute" => Account,
        (get) unmute: "accounts/{}/unmute" => Account,
        (get) get_notification: "notifications/{}" => Notification,
        (get) get_status: "statuses/{}" => Status,
        (get) get_context: "statuses/{}/context" => Context,
        (get) get_card: "statuses/{}/card" => Card,
        (post) reblog: "statuses/{}/reblog" => Status,
        (post) unreblog: "statuses/{}/unreblog" => Status,
        (post) favourite: "statuses/{}/favourite" => Status,
        (post) unfavourite: "statuses/{}/unfavourite" => Status,
        (delete) delete_status: "statuses/{}" => Empty,
    }

    pub fn update_credentials(&self, changes: CredientialsBuilder)
        -> Result<Account>
    {

        let url = self.route("/api/v1/accounts/update_credentials");
        let response = self.client.patch(&url)
            .headers(self.headers.clone())
            .multipart(changes.into_form()?)
            .send()?;

        let status = response.status().clone();

        if status.is_client_error() {
            return Err(Error::Client(status));
        } else if status.is_server_error() {
            return Err(Error::Server(status));
        }

        deserialise(response)
    }

    /// Post a new status to the account.
    pub fn new_status(&self, status: StatusBuilder) -> Result<Status> {

        let response = self.client.post(&self.route("/api/v1/statuses"))
            .headers(self.headers.clone())
            .json(&status)
            .send()?;

        deserialise(response)
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
    ///
    /// # Example
    ///
    /// ```no_run
    /// # extern crate mammut;
    /// # use mammut::{Data, Mastodon};
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// let client = Mastodon::from_data(data);
    /// let statuses = client.statuses("user-id", None)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// # extern crate mammut;
    /// # use mammut::{Data, Mastodon, StatusesRequest};
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # let data = Data {
    /// #   base: "".into(),
    /// #   client_id: "".into(),
    /// #   client_secret: "".into(),
    /// #   redirect: "".into(),
    /// #   token: "".into(),
    /// # };
    /// let client = Mastodon::from_data(data);
    /// let request = StatusesRequest::default()
    ///                               .only_media();
    /// let statuses = client.statuses("user-id", request)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn statuses<'a, S>(&self, id: &str, request: S) -> Result<Page<Status>>
            where S: Into<Option<StatusesRequest<'a>>>
    {
        let mut url = format!("{}/api/v1/accounts/{}/statuses", self.base, id);

        if let Some(request) = request.into() {
            url = format!("{}{}", url, request.to_querystring());
        }

        let response = self.client.get(&url)
            .headers(self.headers.clone())
            .send()?;

        Page::new(self, response)
    }

    /// Returns the client account's relationship to a list of other accounts.
    /// Such as whether they follow them or vice versa.
    pub fn relationships(&self, ids: &[&str]) -> Result<Page<Relationship>> {
        let mut url = self.route("/api/v1/accounts/relationships?");

        if ids.len() == 1 {
            url += "id=";
            url += &ids[0];
        } else {
            for id in ids {
                url += "id[]=";
                url += &id;
                url += "&";
            }
            url.pop();
        }

        let response = self.client.get(&url)
            .headers(self.headers.clone())
            .send()?;

        Page::new(self, response)
    }

    /// Search for accounts by their name.
    /// Will lookup an account remotely if the search term is in the
    /// `username@domain` format and not yet in the database.
    pub fn search_accounts(&self,
                           query: &str,
                           limit: Option<u64>,
                           following: bool)
        -> Result<Page<Account>>
    {
        let url = format!("{}/api/v1/accounts/search?q={}&limit={}&following={}",
                          self.base,
                          query,
                          limit.unwrap_or(40),
                          following);

        let response = self.client.get(&url)
            .headers(self.headers.clone())
            .send()?;

        Page::new(self, response)
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
    HttpError, Http,
    IoError, Io,
    SerdeError, Serde,
    UrlError, Url,
}

// Convert the HTTP response body from JSON. Pass up deserialization errors
// transparently.
fn deserialise<T: for<'de> serde::Deserialize<'de>>(mut response: Response)
    -> Result<T>
{
    use std::io::Read;

    let mut vec = Vec::new();
    response.read_to_end(&mut vec)?;

    match json::from_slice(&vec) {
        Ok(t) => Ok(t),
        // If deserializing into the desired type fails try again to
        // see if this is an error response.
        Err(e) => {
            if let Ok(error) = json::from_slice(&vec) {
                return Err(Error::Api(error));
            }
            Err(e.into())
        },
    }
}
