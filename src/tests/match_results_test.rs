use super::super::models::match_result::MatchResult;

fn get_model() -> MatchResult {
    MatchResult::new(
        vec![String::from("Match: 02"),String::from("Person A vs Person C")],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_initialize_match_result() {
        let match_result = super::get_model();
        assert_eq!(match_result.match_id, "02".to_string());
        assert_eq!(match_result.winner.name, "Person A".to_string());
        assert_eq!(match_result.loser.name, "Person C".to_string());
    }

    #[test]
    fn should_get_match_score() {
        let match_result = super::get_model();
        let result: (String,String) = match_result.get_match_score();

        assert_eq!(result.0,"Person A defeated Person C".to_string());
        assert_eq!(result.1,"1 sets to 0".to_string());
    }
}
