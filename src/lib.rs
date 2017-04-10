//! # Mammut: API Wrapper around the Mastodon API.
//!
//! Most of the api is documented on [Mastodon's
//! github](https://github.com/tootsuite/mastodon/blob/master/docs/Using-the-API/API.md#tag)
#![deny(unused_must_use)]

#[cfg_attr(test, deny(warnings))]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json as json;
extern crate chrono;
extern crate reqwest;
extern crate serde;

pub mod apps;
pub mod status_builder;
pub mod entities;

use json::Error as SerdeError;
use reqwest::Error as HttpError;
use reqwest::Client;
use reqwest::header::{Authorization, Bearer, Headers};

use entities::prelude::*;
use status_builder::StatusBuilder;

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! methods {
    ($($method:ident,)+) => {
        $(
            fn $method<T: serde::Deserialize>(&self, url: String)
            -> Result<T>
            {
                Ok(self.client.$method(&url)
                   .headers(self.access_token.clone().unwrap())
                   .send()?
                   .json()?)
            }
         )+
    };
}

macro_rules! route {

    ((post ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        /// Requires `access_token` or will return error.
        pub fn $name(&self, $($param: $typ,)*) -> Result<$ret> {
            self.has_access_token()?;

            let form_data = json!({
                $(
                    stringify!($param): $param,
                    )*
            });

            Ok(self.client.post(&self.route(concat!("/api/v1/", $url)))
                          .headers(self.access_token.clone().unwrap())
                          .form(&form_data)
                          .send()?
                          .json()?)
        }
        route!{$($rest)*}
    };

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        /// Requires `access_token` or will return error.
        pub fn $name(&self) -> Result<$ret> {
            self.has_access_token()?;

            self.$method(self.route(concat!("/api/v1/", $url)))
        }

        route!{$($rest)*}
    };

    () => {}
}

macro_rules! route_id {

    ($(($method:ident) $name:ident: $url:expr => $ret:ty,)*) => {
        $(
            /// Requires `access_token` or will return error.
            pub fn $name(&self, id: u64) -> Result<$ret> {
                self.has_access_token()?;


                self.$method(self.route(&format!(concat!("/api/v1/", $url), id)))
            }
         )*
    }

}

#[derive(Clone, Debug)]
pub struct Mastodon {
    base_url: String,
    client: Client,
    client_id: Option<String>,
    client_secret: Option<String>,
    redirect_uri: Option<String>,
    access_token: Option<Headers>,
    id: Option<u64>,
}

#[derive(Deserialize)]
struct OAuth {
    client_id: String,
    client_secret: String,
    id: u64,
    redirect_uri: String,
}

#[derive(Debug)]
pub enum Error {
    Serde(SerdeError),
    Http(HttpError),
    ClientIdRequired,
    ClientSecretRequired,
    AccessTokenRequired,
}

impl Mastodon {
    /// Inits new Mastodon object. `base_url` is expected in the following
    /// format `https://mastodon.social` with no leading forward slash.
    ///
    /// ```
    /// use mammut::Mastodon;
    ///
    /// let mastodon = Mastodon::new("https://mastodon.social").unwrap();
    /// ```
    pub fn new<I: Into<String>>(base_url: I) -> Result<Self> {
        Ok(Mastodon {
            base_url: base_url.into(),
            client: Client::new()?,
            client_id: None,
            client_secret: None,
            redirect_uri: None,
            access_token: None,
            id: None,
        })
    }

    /// Register the application with the server from `base_url`.
    ///
    /// ```
    /// # extern crate mammut;
    /// # fn main() {
    /// #    try().unwrap();
    /// # }
    ///
    /// # fn try() -> mammut::Result<()> {
    /// use mammut::Mastodon;
    /// use mammut::apps::{AppBuilder, Scope};
    ///
    /// let app = AppBuilder {
    ///     client_name: "mammut_test",
    ///     redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
    ///     scopes: Scope::Read,
    ///     website: None,
    /// };
    ///
    /// let mut mastodon = Mastodon::new("https://mastodon.social")?;
    /// mastodon.register(app)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn register(&mut self, app_builder: apps::AppBuilder) -> Result<()> {
        let url = self.route("/api/v1/apps");

        let app: OAuth = self.client.post(&url).form(&app_builder).send()?.json()?;

        self.id = Some(app.id);
        self.client_id = Some(app.client_id);
        self.client_secret = Some(app.client_secret);
        self.redirect_uri = Some(app.redirect_uri);

        Ok(())
    }

    /// Returns the full url needed for authorisation. This needs to be opened
    /// in a browser.
    pub fn authorise(&mut self) -> Result<String> {
        self.is_registered()?;

        let url = format!(
            "{}/oauth/authorize?client_id={}&redirect_uri={}&response_type=code",
            self.base_url,
            self.client_id.clone().unwrap(),
            self.redirect_uri.clone().unwrap(),
        );

        Ok(url)
    }

    /// Set `access_token` required to use any method about the user.
    pub fn set_access_token(&mut self, access_token: String) {
        let mut headers = Headers::new();

        headers.set(Authorization(Bearer { token: access_token }));

        self.access_token = Some(headers);
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
        (post (uri: String,)) follows: "follows" => Account,
        (post) clear_notifications: "notifications/clear" => Empty,
        (post (file: Vec<u8>,)) media: "media" => Attachment,
        (post (account_id: u64, status_ids: Vec<u64>, comment: String,)) report:
            "reports" => Report,
        (post (q: String, resolve: bool,)) search: "search" => SearchResult,
        (post (status: StatusBuilder,)) new_status: "statuses" => Status,
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

    pub fn get_public_timeline(&self, local: bool) -> Result<Vec<Status>> {
        self.has_access_token()?;

        let mut url = self.route("/api/v1/timelines/public");

        if local {
            url += "?local=1";
        }

        self.get(url)
    }

    pub fn get_tagged_timeline(&self, hashtag: String, local: bool) -> Result<Vec<Status>> {
        self.has_access_token()?;

        let mut url = self.route("/api/v1/timelines/tag/");
        url += &hashtag;

        if local {
            url += "?local=1";
        }

        self.get(url)
    }

    pub fn statuses(&self, id: u64, only_media: bool, exclude_replies: bool)
        -> Result<Vec<Status>>
    {
        self.has_access_token()?;
        let mut url = format!("{}/api/v1/accounts/{}/statuses", self.base_url, id);

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


    pub fn relationships(&self, ids: &[u64]) -> Result<Vec<Relationship>> {
        self.has_access_token()?;

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

    // TODO: Add a limit fn
    pub fn search_accounts(&self, query: &str) -> Result<Vec<Account>> {
        self.has_access_token()?;
        self.get(format!("{}/api/v1/accounts/search?q={}", self.base_url, query))
    }

    pub fn instance(&self) -> Result<Instance> {
        self.is_registered()?;

        self.get(self.route("/api/v1/instance"))
    }


    fn has_access_token(&self) -> Result<()> {
        if self.access_token.is_none() {
            Err(Error::AccessTokenRequired)
        } else {
            Ok(())
        }
    }

    fn is_registered(&self) -> Result<()> {
        if self.client_id.is_none() {
            Err(Error::ClientIdRequired)
        } else if self.client_secret.is_none() {
            Err(Error::ClientSecretRequired)
        } else {
            Ok(())
        }
    }

    methods![get, post, delete,];

    fn route(&self, url: &str) -> String {
        let mut s = self.base_url.clone();
        s += url;
        s
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
}
