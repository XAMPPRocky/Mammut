extern crate mammut;
extern crate toml;

use std::{
    error::Error,
    fs,
    io,
};

use self::mammut::{
    apps::{
        AppBuilder,
        Scopes
    },
    Mastodon,
    Registration
};

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    register()?;
    Ok(())
}


#[allow(dead_code)]
pub fn get_mastodon_data() -> Result<Mastodon, Box<dyn Error>> {
    if let Ok(config) = fs::read_to_string("mastodon-data.toml") {
        Ok(Mastodon::from_data(toml::from_str(&config)?))
    } else {
        register()
    }
}

pub fn register() -> Result<Mastodon, Box<dyn Error>> {
    let app = AppBuilder {
        client_name: "mammut-examples",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::All,
        website: Some("https://github.com/Aaronepower/mammut"),
    };

    let website = read_line("Please enter your mastodon instance url:")?;
    let mut registration = Registration::new(website.trim());
    registration.register(app)?;
    let url = registration.authorise()?;

    println!("Click this link to authorize on Mastodon: {}", url);
    let input = read_line("Paste the returned authorization code: ")?;

    let code = input.trim();
    let mastodon = registration.create_access_token(code.to_string())?;

    // Save app data for using on the next run.
    let toml = toml::to_string(&*mastodon)?;
    fs::write("mastodon-data.toml", toml.as_bytes())?;

    Ok(mastodon)
}

pub fn read_line(message: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", message);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}

