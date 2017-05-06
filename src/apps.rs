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
    pub client_name: &'a str,
    pub redirect_uris: &'a str,
    pub scopes: Scope,
    #[serde(skip_serializing_if="Option::is_none")]
    pub website: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Scope {
    /// All Permissions, equiavlent to `read write follow`
    #[serde(rename = "read write follow")]
    All,
    /// Only permission to add and remove followers.
    #[serde(rename = "follow")]
    Follow,
    /// Read only permissions.
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "read follow")]
    ReadFollow,
    #[serde(rename = "read write")]
    ReadWrite,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "write follow")]
    WriteFollow,
}

impl ::std::fmt::Display for Scope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
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
