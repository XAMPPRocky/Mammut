mod register;

use std::error;

fn main() -> Result<(), Box<error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let you = mastodon.verify_credentials()?;

    println!("{:#?}", you);

    Ok(())
}
