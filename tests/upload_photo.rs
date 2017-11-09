extern crate mammut;
extern crate dotenv;

use std::env;

use mammut::{Data, Mastodon};
use dotenv::dotenv;

#[test]
fn upload_photo() {
    dotenv().ok();
    run().unwrap();
}

fn run() -> mammut::Result<()> {

    let data = Data {
        base: String::from(env::var("BASE").unwrap()),
        client_id: String::from(env::var("CLIENT_ID").unwrap()),
        client_secret: String::from(env::var("CLIENT_SECRET").unwrap()),
        redirect: String::from(env::var("REDIRECT").unwrap()),
        token: String::from(env::var("TOKEN").unwrap()),
    };

    let mastodon = Mastodon::from_data(data);

    mastodon.media("tests/test.png".into())?;
    Ok(())
}
