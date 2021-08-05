use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use csv::Writer;

struct PGN {
    event: String,
    site: String,
    date: String,
    white: String,
    black: String,
    game_result: String,
    white_elo: String,
    black_elo: String,
    time_control: String,
    termination: String,
    moves: String
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        print!("filename was not specified");
        return; 
    }

    if args.len() <= 2 {
        print!("output file was not specified");
        return; 
    }

    let input: String = args[1].clone();
    let output: String = args[2].clone();

    let handle = match open_file(input) {
        Ok(h) => h,
        Err(e) => {
            print!("unable to open file due to {:?}", e);
            return;
        }
    };

    let output_handle = match open_file(output) {
        Ok(h) => h,
        Err(e) => {
            print!("unable to open file due to {:?}", e);
            return;
        }
    };


    parse_file(handle, output_handle);
}

fn open_file(name: String) -> Result<File, io::Error> {
    let handle = match File::open(name) {
        Ok(s) => s,
        Err(e) => {
            return Err(e);
        }
    };

    return Ok(handle);
}

fn parse_file(file: File, output: File) {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut pgn = PGN::default();
    let mut whitespace_found: bool = false;

    for line in lines {

        let _line = line.unwrap();
        let stripped = strip_line(_line.clone());

        if whitespace_found {
            pgn.moves = _line.clone();
            whitespace_found = false;
        }

        if _line.clone().is_empty() {
            whitespace_found = true;
            continue;
        }

        if !whitespace_found {
            let split = stripped.split(' ').collect::<Vec<&str>>();

            match split[0] {
                "Event"=> pgn.event = get_value(split),
                "Site"=> pgn.site = get_value(split),
                "Date"=> pgn.date = get_value(split),
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

    write_to_file(output, pgn);
}

fn write_to_file(file: File, pgn: PGN) {
}

fn get_value(split: Vec<&str>) -> String {
    split[1..].into_iter().map(|x| x.to_string() + " ").collect::<String>()
}

fn strip_line(line: String) -> String {
    let s = line.replace(&['[', ']', '"'][..], "");
    return s.to_string();
}
