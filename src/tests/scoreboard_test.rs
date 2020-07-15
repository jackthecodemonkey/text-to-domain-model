use super::super::models::score_data_parser::ScoreDataParser;
use super::super::models::scoreboard::Scoreboard;
use std::path::Path;

fn get_scoreboard<P>(path: P) -> Scoreboard
where
    P: AsRef<Path>,
{
    let data = ScoreDataParser::get_parsed_data(path);
    Scoreboard::new(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_initialize_scoreboard() {
        let score_board = super::get_scoreboard("src/data_source/test_data.txt");
        let result = score_board.match_results.get(0).unwrap();

        assert_eq!(result.match_id, "15".to_string());
        assert_eq!(result.match_title, "Person F vs Person G".to_string());
    }

    #[test]
    fn should_have_winner_details() {
        let score_board = super::get_scoreboard("src/data_source/test_data.txt");
        let result = score_board.match_results.get(0).unwrap();

        assert_eq!(result.winner.name, "Person F".to_string());
        assert_eq!(result.winner.games, 12);
        assert_eq!(result.winner.set_score, 2);
        assert_eq!(result.winner.won, true);
    }
    
    #[test]
    fn should_have_loser_details() {
        let score_board = super::get_scoreboard("src/data_source/test_data.txt");
        let result = score_board.match_results.get(0).unwrap();

        assert_eq!(result.loser.name, "Person G".to_string());
        assert_eq!(result.loser.games, 0);
        assert_eq!(result.loser.set_score, 0);
        assert_eq!(result.loser.won, false);
    }

}
