//! A module containing everything relating to a account returned from the api.

use std::path::Path;

use chrono::prelude::*;
use reqwest::multipart::Form;

use crate::Result;

/// A struct representing an Account.
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    /// Equals `username` for local users, includes `@domain` for remote ones.
    pub acct: String,
    /// URL to the avatar image
    pub avatar: String,
    /// URL to the avatar static image (gif)
    pub avatar_static: String,
    /// The time the account was created.
    pub created_at: DateTime<Utc>,
    /// The account's display name.
    pub display_name: String,
    /// The number of followers for the account.
    pub followers_count: u64,
    /// The number of accounts the given account is following.
    pub following_count: u64,
    /// URL to the header image.
    pub header: String,
    /// URL to the header static image (gif).
    pub header_static: String,
    /// The ID of the account.
    pub id: String,
    /// Boolean for when the account cannot be followed without waiting for
    /// approval first.
    pub locked: bool,
    /// Biography of user.
    pub note: String,
    /// The number of statuses the account has made.
    pub statuses_count: u64,
    /// URL of the user's profile page (can be remote).
    pub url: String,
    /// The username of the account.
    pub username: String,
    /// An extra attribute given from `verify_credentials` giving defaults about
    /// a user
    pub source: Option<Source>,
    /// If the owner decided to switch accounts, new account is in
    /// this attribute
    pub moved: Option<Box<Account>>,
}

/// An extra object given from `verify_credentials` giving defaults about a user
#[derive(Debug, Clone, Deserialize)]
pub struct Source {
    privacy: crate::status_builder::Visibility,
    sensitive: bool,
    note: String,
}

pub struct CredientialsBuilder<'a> {
    display_name: Option<&'a str>,
    note: Option<&'a str>,
    avatar: Option<&'a Path>,
    header: Option<&'a Path>,
}

impl<'a> CredientialsBuilder<'a> {
    pub fn into_form(self) -> Result<Form> {
        let mut form = Form::new();
        macro_rules! add_to_form {
            ($key:ident : Text; $($rest:tt)*) => {{
                if let Some(val) = self.$key {
                    form = form.text(stringify!($key), val.to_owned());
                }

                add_to_form!{$($rest)*}
            }};

            ($key:ident : File; $($rest:tt)*) => {{
                if let Some(val) = self.$key {
                    form = form.file(stringify!($key), val)?;
                }

                add_to_form!{$($rest)*}
            }};

            () => {}
        }

        add_to_form! {
            display_name: Text;
            note: Text;
            avatar: File;
            header: File;
        }

        Ok(form)
    }
}
