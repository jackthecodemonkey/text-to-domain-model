#[derive(Debug,Clone)]
pub struct Player {
    pub name: String,
    pub games: u8,
    pub set_score: u8,
    pub won: bool,
}

impl Player {
    pub fn new(name: String, games: u8, set_score: u8, won: bool) -> Self {
        Player {
            name,
            games,
            set_score,
            won,
        }
    }
}
