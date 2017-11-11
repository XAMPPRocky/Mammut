//! Module representing cards of statuses.

/// A card of a status.
#[derive(Debug, Clone, Deserialize)]
pub struct Card {
    /// The url associated with the card.
    pub url: String,
    /// The title of the card.
    pub title: String,
    /// The card description.
    pub description: String,
    /// The image associated with the card, if any.
    pub image: String,
}
