use super::data_source::DataSource;
use super::match_result::MatchResult;

#[derive(Debug)]
pub struct Scoreboard {
    pub match_results: Vec<MatchResult>,
}

impl Scoreboard {
    pub fn new(match_result: DataSource) -> Self {
        let mut match_results = vec![];
        let mut thread_handles = vec![];

        for result in match_result.data {
            thread_handles.push(
                std::thread::spawn(move || {
                    MatchResult::new(result.header, result.score)
                })
            )            
        }

        for handle in thread_handles {
            let result = handle.join().unwrap();
            match_results.push(result);
        }

        Scoreboard { match_results }
    }

    pub fn show_score_match(&self,id: &String) -> Option<(String,String)> {
        let match_found = self.match_results.iter().find(|match_result| &match_result.match_id == id);
        match match_found {
            Some(result) => {
                Some(result.get_match_score())
            },
            None => None
        }
    }

    pub fn show_games_player(&self, player: &String) -> Vec<String> {
        let games: Vec<String> = self.match_results
            .iter()
            .map(|result| result.find_player(player))
            .flat_map(|e| e)
            .map(|(winner,loser)| {
                let mut games = String::from(winner.games.to_string()); 
                games.push_str(" "); 
                games.push_str(loser.games.to_string().as_str()); 
                games 
            }).collect();
        games
    } 
}
