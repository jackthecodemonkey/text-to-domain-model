use super::super::models::scores::Scores;

fn get_scores() -> ((u8, u8), (u8, u8)) {
    let scores = Scores::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    scores.get_scores()
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_get_scores() {
        let scores = super::get_scores();
        let games = scores.0;
        let sets = scores.1;

        assert_eq!(games.0,17);
        assert_eq!(games.1,11);

        assert_eq!(sets.0,2);
        assert_eq!(sets.1,1);
    }
}
