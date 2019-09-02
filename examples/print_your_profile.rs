mod register;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let you = mastodon.verify_credentials()?;

    println!("{:#?}", you);

    Ok(())
}
