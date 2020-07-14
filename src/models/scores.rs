#[derive(Debug, Clone)]
pub struct Scores {
    pub scores: Vec<u8>,
}

impl Scores {
    pub fn new(scores: Vec<u8>) -> Self {
        Scores { scores }
    }

    pub fn get_scores(&self) -> ((u8, u8), (u8, u8)) {
        let mut player: u8 = 0;
        let mut opponent: u8 = 0;

        let mut player_point: u8 = 0;
        let mut opponent_point: u8 = 0;

        let mut player_game: u8 = 0;
        let mut opponent_game: u8 = 0;

        let mut player_total_game: u8 = 0;
        let mut opponent_total_game: u8 = 0;

        let mut is_duece = false;

        for score in &self.scores {
            if *score == 0 {
                player_point += 1
            }

            if *score == 1 {
                opponent_point += 1
            }

            if !is_duece
                && player_point != opponent_point
                && (player_point == 4 || opponent_point == 4)
            {
                if player_point == 4 {
                    player_game += 1;
                    player_total_game += 1;
                }
                if opponent_point == 4 {
                    opponent_game += 1;
                    opponent_total_game += 1;
                }
                player_point = 0;
                opponent_point = 0;
            }

            if player_point == 3 && player_point == opponent_point {
                is_duece = true;
                player_point = 0;
                opponent_point = 0;
            }

            if is_duece {
                if player_point + 2 == opponent_point {
                    opponent_game += 1;
                    opponent_total_game += 1;
                    player_point = 0;
                    opponent_point = 0;
                    is_duece = false;
                } else if opponent_point + 2 == player_point {
                    player_game += 1;
                    player_total_game += 1;
                    player_point = 0;
                    opponent_point = 0;
                    is_duece = false;
                }
            }

            if player_game == 6 {
                player += 1;
            }

            if opponent_game == 6 {
                opponent += 1;
            }

            if player_game == 6 || opponent_game == 6 {
                player_game = 0;
                opponent_game = 0;
                player_point = 0;
                opponent_point = 0;
                is_duece = false;
            }
        }

        if player > opponent {
            ((player_total_game, opponent_total_game), (player, opponent))
        } else {
            ((opponent_total_game,player_total_game), (opponent, player))
        }
    }
}