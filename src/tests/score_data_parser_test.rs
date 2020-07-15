use super::super::models::score_data_parser::ScoreDataParser;
use super::super::models::data_source::DataSource;
use std::path::Path;

fn get_data<P>(path: P) -> DataSource
where
    P: AsRef<Path>,
{
    ScoreDataParser::get_parsed_data(path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_parse_data_from_text_file() {
        let parsed_data = super::get_data("src/data_source/test_data.txt");

        assert_eq!(parsed_data.data.len(),1);
    }

    #[test]
    fn should_have_header_and_score() {
        let parsed_data = super::get_data("src/data_source/test_data.txt");
        let result = parsed_data.data.get(0).unwrap();

        assert_eq!(result.header.len(),2);
        assert_eq!(result.score.len(),48);
    }
}
