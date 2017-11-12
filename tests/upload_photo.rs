extern crate mammut;
extern crate dotenv;

use std::env;

use mammut::{Data, Mastodon};
use dotenv::dotenv;

// Do not run this test by default because it requires a real Mastodon
// connection setup.
#[test]
#[ignore]
fn upload_photo() {
    dotenv().ok();
    run().unwrap();
}

fn run() -> mammut::Result<()> {

    let data = Data {
        base: env::var("BASE").unwrap().into(),
        client_id: env::var("CLIENT_ID").unwrap().into(),
        client_secret: env::var("CLIENT_SECRET").unwrap().into(),
        redirect: env::var("REDIRECT").unwrap().into(),
        token: env::var("TOKEN").unwrap().into(),
    };

    let mastodon = Mastodon::from_data(data);

    mastodon.media("tests/test.png".into())?;
    Ok(())
}
