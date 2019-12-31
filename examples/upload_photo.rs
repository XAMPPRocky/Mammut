mod register;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let input = register::read_line("Enter the path to the photo you'd like to post: ")?;

    mastodon.media(input.into())?;

    Ok(())
}
