mod register;

use std::error;

fn main() -> Result<(), Box<error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let file_name = register::read_line("Enter the path to the photo you'd like to post: ")?;
    let alt_text = register::read_line("Enter the description text for the photo: ")?;

    mastodon.media(file_name.into(), Some(alt_text.into()), None)?;

    Ok(())
}
