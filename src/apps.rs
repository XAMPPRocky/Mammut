use std::fmt;

/// Builder struct for defining your application.
/// ```
/// let app = AppBuilder {
///     client_name: "mammut_test",
///     redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
///     scopes: Scope::Read,
///     website: None,
/// };
/// ```
#[derive(Debug, Default, Serialize)]
pub struct AppBuilder<'a> {
    /// Name of the application. Will be displayed when the user is deciding to
    /// grant permission.
    pub client_name: &'a str,
    /// Where the user should be redirected after authorization
    /// (for no redirect, use `urn:ietf:wg:oauth:2.0:oob`)
    pub redirect_uris: &'a str,
    /// Permission scope of the application.
    pub scopes: Scope,
    /// URL to the homepage of your application.
    #[serde(skip_serializing_if="Option::is_none")]
    pub website: Option<&'a str>,
}

/// Permission scope of the application.
/// [Details on what each permission provides](//github.com/tootsuite/documentation/blob/master/Using-the-API/OAuth-details.md)
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Scope {
    /// All Permissions, equivalent to `read write follow`
    #[serde(rename = "read write follow")]
    All,
    /// Only permission to add and remove followers.
    #[serde(rename = "follow")]
    Follow,
    /// Read only permissions.
    #[serde(rename = "read")]
    Read,
    /// Read & Follow permissions.
    #[serde(rename = "read follow")]
    ReadFollow,
    /// Read & Write permissions.
    #[serde(rename = "read write")]
    ReadWrite,
    /// Write only permissions.
    #[serde(rename = "write")]
    Write,
    /// Write & Follow permissions.
    #[serde(rename = "write follow")]
    WriteFollow,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Scope::*;
        write!(f, "{}", match *self {
            All => "read write follow",
            Follow => "follow",
            Read => "read",
            ReadFollow => "read follow",
            ReadWrite => "read write",
            Write => "write",
            WriteFollow => "write follow"
        })
    }
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Read
    }
}
