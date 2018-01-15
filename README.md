# Mammut. A API Wrapper for the Mastodon API.

## [Documentation](https://docs.rs/mammut/)

A wrapper around the [API](https://github.com/tootsuite/mastodon/blob/master/docs/Using-the-API/API.md#tag) for [Mastodon](https://mastodon.social/)

```rust
extern crate mammut;
use mammut::Registration;
use mammut::apps::{AppBuilder, Scope};

fn main() {
   run().unwrap();
}

fn run() -> mammut::Result<()> {
    let app = AppBuilder {
        client_name: "mammut_test",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::Read,
        website: None,
    };

    let mut registration = Registration::new("https://mastodon.social");
    registration.register(app)?;
    let url = registration.authorise()?;
    // Here you now need to open the url in the browser
    // And handle a the redirect url coming back with the code.
    let code = String::from("RETURNED_FROM_BROWSER");
    let mastodon = registration.create_access_token(code)?;

    println!("{:?}", mastodon.get_home_timeline()?);
    Ok(())
}
```
