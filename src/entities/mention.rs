pub struct Mention {
    /// URL of user's profile (can be remote)
    pub url: String,
    /// The username of the account
    pub username: String,
    /// Equals username for local users, includes `@domain` for remote ones
    pub acct: String,
    /// Account ID
    pub id: String,
}
