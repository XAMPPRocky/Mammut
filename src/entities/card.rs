#[derive(Deserialize)]
pub struct Card {
    pub url: String,
    pub title: String,
    pub description: String,
    pub image: String,
}
