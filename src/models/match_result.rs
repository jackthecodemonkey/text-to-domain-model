use super::header_lines::HeaderLines;
use super::player::Player;
use super::scores::Scores;

#[derive(Debug)]
pub struct MatchResult {
    pub match_id: String,
    pub match_title: String,
    pub winner: Player,
    pub loser: Player,
}

impl MatchResult {
    pub fn new(match_title: Vec<String>, scores: Vec<u8>) -> Self {
        let sc = Scores::new(scores);
        let match_scores = sc.get_scores();

        let game_score = match_scores.0;
        let set_score = match_scores.1;

        let header = MatchResult::get_header_lines(match_title);
        let players = MatchResult::get_players(header.title.clone(), game_score, set_score);

        MatchResult {
            match_id: header.id,
            match_title: header.title,
            winner: players.0.clone(),
            loser: players.1,
        }
    }

    fn get_header_lines(header: Vec<String>) -> HeaderLines {
        let match_id = header.get(0).unwrap();
        let id: Vec<&str> = match_id.split(":").collect();
        let id = id.get(1).unwrap();
        let match_title = header.get(1).unwrap();
        return HeaderLines::new(
            id.trim_start().trim_end().to_string(),
            match_title.to_owned(),
        );
    }

    fn get_players(players: String, game_score: (u8, u8), set_score: (u8, u8)) -> (Player, Player) {
        let players: Vec<&str> = players.split("vs").collect();
        let player = players.get(0).unwrap();
        let opponent = players.get(1).unwrap();
        let player_set = set_score.0;
        let opponent_set = set_score.1;
        let player_won = player_set > opponent_set;
        (
            Player::new(
                player.trim_start().trim_end().to_string(),
                game_score.0,
                set_score.0,
                player_won,
            ),
            Player::new(
                opponent.trim_start().trim_end().to_string(),
                game_score.1,
                set_score.1,
                !player_won,
            ),
        )
    }
}

impl MatchResult {
    pub fn find_player(&self, player: &String) -> Option<(&Player,&Player)> {
        if self.winner.name == *player {
            return Some((&self.winner, &self.loser))
        } else if self.loser.name == *player {
            return Some((&self.loser, &self.winner))
        } 
        None
    }

    pub fn get_match_score(&self) -> (String,String) {
        let mut summary = String::from("");
        summary.push_str(self.winner.name.as_str());
        summary.push_str(" defeated ");
        summary.push_str(self.loser.name.as_str());

        let mut result = String::from("");
        result.push_str(self.winner.set_score.to_string().as_str());
        result.push_str(" sets to ");
        result.push_str(self.loser.set_score.to_string().as_str());
        
        (summary,result)
    }
}
