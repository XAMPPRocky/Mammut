mod register;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let input = register::read_line("Enter the account id you'd like to follow: ")?;
    let new_follow = mastodon.follow(input.trim())?;

    println!("{:#?}", new_follow);

    Ok(())
}
