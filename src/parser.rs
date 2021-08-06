use std::io::BufReader;
use std::fs::File;
use serde::Serialize;
use std::io::prelude::*;

#[derive(Serialize, Clone)]
pub struct PGN {
    pub event: String,
    pub site: String,
    pub date: String,
    pub white: String,
    pub black: String,
    pub game_result: String,
    pub white_elo: String,
    pub black_elo: String,
    pub time_control: String,
    pub termination: String,
    pub moves: String
}

impl Default for PGN {
    fn default() -> PGN {
        PGN {
            event: "".to_string(),
            site: "".to_string(),
            date: "".to_string(),
            white: "".to_string(),
            black: "".to_string(),
            game_result: "".to_string(),
            white_elo: "".to_string(),
            black_elo: "".to_string(),
            time_control: "".to_string(),
            termination: "".to_string(),
            moves: "".to_string(),
        }
    }
}

pub fn parse_file(file: File) -> Vec<PGN> {
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect();
    let line_count = lines.len();
    let mut out: Vec<PGN> = Vec::new();
    let mut pgn = PGN::default();
    let mut whitespace_found: bool = false;
    let mut moves_written: bool = false;

    for (i, line) in lines.into_iter().enumerate() {
        let _line = line.unwrap();
        let stripped = strip_line(_line.clone());

        if moves_written {
            assert!(_line.clone().is_empty());
            whitespace_found = false;
            moves_written = false;
            out.push(pgn.clone());
            pgn = PGN::default();
            continue;
        }

        if whitespace_found {
            pgn.moves = _line.clone();
            moves_written = true;

            if i >= line_count {
                out.push(pgn.clone());
            }

            continue;
        }

        if _line.clone().is_empty() {
           whitespace_found = true;
           continue;
        }

        if !whitespace_found { 
            let split = stripped.split(' ').collect::<Vec<&str>>();

            let target: String = split[0].chars().filter(|c| !c.is_whitespace()).collect();

            match target.as_str() {
                "Event"=> pgn.event = get_value(split),
                "Site"=> pgn.site = get_value(split),
                "UTCDate"=> pgn.date = get_value(split),
                "White"=> pgn.white = get_value(split),
                "Black"=> pgn.black = get_value(split),
                "Result"=> pgn.game_result = get_value(split),
                "WhiteElo"=> pgn.white_elo = get_value(split),
                "BlackElo"=> pgn.black_elo = get_value(split),
                "TimeControl"=> pgn.time_control = get_value(split),
                "Termination"=> pgn.termination = get_value(split) ,

                _ => ()
            }
        }
    }

    return out;
}

fn get_value(split: Vec<&str>) -> String {
    split[1..].into_iter().map(|x| x.to_string() + " ").collect::<String>()
}

fn strip_line(line: String) -> String {
    let s = line.replace(&['[', ']', '"'][..], "");
    return s.to_string();
}
