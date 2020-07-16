extern crate regex;
mod models;
mod common;
#[cfg(test)]
mod tests;
use common::read_input::read_input_from_user;
use models::{score_data_parser::ScoreDataParser, scoreboard::Scoreboard};
use regex::Regex;

#[derive(Debug)]
pub enum Command {
    None,
    ShowScoreMatch(String),
    ShowGamesPlayer(String),
}

pub struct View;
impl View {
    pub fn get_command_from_user() -> Command {
        let input_text = read_input_from_user();
        let trimmed = input_text.trim();
        let score_match = Regex::new(r"^Score Match ([\w+\s+]+)$").unwrap();
        let game_player = Regex::new(r"^Games Player ([\w+\s+]+)$").unwrap();
        let mut command = Command::None;
        for cap in score_match.captures_iter(trimmed) {
            if let Some(val) = cap.get(1) {
                command = Command::ShowScoreMatch(val.as_str().to_string());
            }
        }
        for cap in game_player.captures_iter(trimmed) {
            if let Some(val) = cap.get(1) {
                command = Command::ShowGamesPlayer(val.as_str().to_string());
            }
        }
        command
    }
}

fn main() {
    let score_data = ScoreDataParser::get_parsed_data("src/data_source/full_tournament.txt");
    let score_board = Scoreboard::new(score_data);

    println!("{:#?}", score_board);

    loop {
        println!("\n1. Query for a match result : Score Match <id>");
        println!("2. Query games for player : Games Player <Player Name>\n");
        println!("****Enter your query****");
        let command: Command = View::get_command_from_user();
        match command {
            Command::ShowGamesPlayer(player_name) => {
                let games: Vec<String> = score_board.show_games_player(&player_name);
                if games.len() == 0 {
                    println!("\nPlayer {:?} not found from match results", player_name);
                } else {
                    for game in games {
                        println!("\n{:?}", game);
                    }
                }
            }
            Command::ShowScoreMatch(match_id) => match score_board.show_score_match(&match_id) {
                Some((summary, result)) => {
                    println!("\n{:?}", summary);
                    println!("{:?}", result)
                }
                None => println!("\nNo match id found."),
            },
            Command::None => {
                println!("You have entered invalid query, please try again.");
            }
        }
    }
}