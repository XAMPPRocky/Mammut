#[derive(Deserialize)]
pub struct Relationship {
    pub following: bool,
    pub followed_by: bool,
    pub blocking: bool,
    pub muting: bool,
    pub requested: bool,
}
