mod register;

use register::MastodonClient;
use std::error;

fn main() -> Result<(), Box<error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let input = register::read_line("Enter the term you'd like to search: ")?;
    let result = mastodon.search_accounts(&input, None, true)?;

    println!("{:#?}", result.initial_items);

    Ok(())
}
