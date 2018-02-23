# Mammut. A API Wrapper for the Mastodon API.

[![crates.io](https://img.shields.io/crates/v/mammut.svg)](https://crates.io/crates/mammut)
[![Docs](https://docs.rs/mammut/badge.svg)](https://docs.rs/mammut)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/mammut.svg)](https://crates.io/crates/mammut)

## [Documentation](https://docs.rs/mammut/)

A wrapper around the [API](https://github.com/tootsuite/mastodon/blob/master/docs/Using-the-API/API.md#tag) for [Mastodon](https://mastodon.social/)

```rust
extern crate mammut;
extern crate toml;

use std::io;
use std::fs::File;
use std::io::prelude::*;

use mammut::{Data, Mastodon, Registration};
use mammut::apps::{AppBuilder, Scopes};

fn main() {
    let mastodon = match File::open("mastodon-data.toml") {
        Ok(mut file) => {
            let mut config = String::new();
            file.read_to_string(&mut config).unwrap();
            let data: Data = toml::from_str(&config).unwrap();
            Mastodon::from_data(data)
        },
        Err(_) => register(),
    };

    let you = mastodon.verify_credentials().unwrap();

    println!("{:#?}", you);
}

fn register() -> Mastodon {
    let app = AppBuilder {
        client_name: "mammut-examples",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::Read,
        website: Some("https://github.com/Aaronepower/mammut"),
    };

    let mut registration = Registration::new("https://mastodon.social");
    registration.register(app).unwrap();;
    let url = registration.authorise().unwrap();

    println!("Click this link to authorize on Mastodon: {}", url);
    println!("Paste the returned authorization code: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let code = input.trim();
    let mastodon = registration.create_access_token(code.to_string()).unwrap();

    // Save app data for using on the next run.
    let toml = toml::to_string(&*mastodon).unwrap();
    let mut file = File::create("mastodon-data.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();

    mastodon
}
```
