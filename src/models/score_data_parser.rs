use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use super::data_source::DataSource;
use super::parsed_result::ParsedResult;

pub struct ScoreDataParser;

impl ScoreDataParser {
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn get_parsed_data<P>(path: P) -> DataSource
    where
        P: AsRef<Path>,
    {
        let mut prev: String = String::from("");
        let mut parsed_result = ParsedResult {
            header: vec![],
            score: vec![],
        };
        let mut data_source = DataSource { data: vec![] };
        if let Ok(lines) = ScoreDataParser::read_lines(path) {
            for line in lines {
                if let Ok(ip) = line {
                    if ip.len() != 0 {
                        match prev.parse::<u8>() {
                            Ok(_) => match ip.parse::<u8>() {
                                Ok(n) => {
                                    parsed_result.score.push(n);
                                }
                                Err(_) => {
                                    data_source.data.push(parsed_result);
                                    parsed_result = ParsedResult {
                                        header: vec![],
                                        score: vec![],
                                    };
                                    parsed_result.header.push(ip.clone());
                                }
                            },
                            Err(_) => match ip.parse::<u8>() {
                                Ok(n) => {
                                    parsed_result.score.push(n);
                                }
                                Err(_) => {
                                    parsed_result.header.push(ip.clone());
                                }
                            },
                        }
                        prev = ip;
                    }
                }
            }
            data_source.data.push(parsed_result);
        }
        data_source
    }
}