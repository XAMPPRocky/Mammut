mod register;

use std::error;

fn main() -> Result<(), Box<error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let input = register::read_line("Enter the account id you'd like to follow: ")?;
    let new_follow = mastodon.follow(input.trim().parse()?)?;

    println!("{:#?}", new_follow);

    Ok(())
}
