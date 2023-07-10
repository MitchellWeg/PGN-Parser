use serde::Serialize;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct PGN {
    pub date: String,
    pub white: String,
    pub black: String,
    pub game_result: String,
    pub white_elo: String,
    pub black_elo: String,
    pub time_control: String,
    pub termination: String,
    pub moves: String,
}

impl Default for PGN {
    fn default() -> PGN {
        PGN {
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

pub struct PGNIterator {
    curr: PGN,
    next: Option<PGN>,
    offset: Option<u64>,
    reader: BufReader<File>,
}

impl PGNIterator {
    pub fn new(file: File) -> PGNIterator {
        let reader = BufReader::new(file);

        PGNIterator {
            curr: PGN::default(),
            next: None,
            offset: None,
            reader,
        }
    }
}

impl Iterator for PGNIterator {
    type Item = PGN;

    fn next(&mut self) -> Option<Self::Item> {
        let off = match self.offset {
            Some(s) => s,
            None => 0,
        };

        let (new_offset, pgn) = parse_lines(&mut self.reader, off);

        self.offset = Some(new_offset);

        pgn
    }
}

pub fn parse_file(file: File) -> PGNIterator {
    PGNIterator::new(file)
}

fn parse_lines(reader: &mut BufReader<File>, offset: u64) -> (u64, Option<PGN>) {
    let mut total_amount_of_bytes_read = offset;
    let mut pgn = PGN::default();
    let mut whitespace_found: bool = false;
    let mut pgn_written: bool = false;

    let mut deq: VecDeque<String> = VecDeque::new();

    // Move the reader offset amount of bytes from the start.
    match reader.seek(SeekFrom::Start(offset)) {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };

    for _line in reader.lines() {
        let line = _line.unwrap();
        let line_bytes = line.chars().count() as u64;
        total_amount_of_bytes_read += line_bytes;

        if line.trim().is_empty() {
            whitespace_found = true;
            continue;
        }

        if pgn_written && whitespace_found {
            break;
        }

        let stripped = strip_line(&line);

        deq.push_back(stripped);

        if whitespace_found {
            for pgn_line in deq.iter() {
                let split = pgn_line.split(' ').collect::<Vec<&str>>();

                match split[0] {
                    "UTCDate" | "Date" => pgn.date = get_value(split),
                    "White" => pgn.white = get_value(split),
                    "Black" => pgn.black = get_value(split),
                    "Result" => pgn.game_result = get_value(split),
                    "WhiteElo" => pgn.white_elo = get_value(split),
                    "BlackElo" => pgn.black_elo = get_value(split),
                    "TimeControl" => pgn.time_control = get_value(split),
                    "Termination" => pgn.termination = get_value(split),

                    _ => pgn.moves = split.join(" "),
                }
            }

            deq.clear();
            pgn_written = true;
        }
    }

    if pgn == PGN::default() {
        return (total_amount_of_bytes_read, None);
    }

    (total_amount_of_bytes_read, Some(pgn))
}

#[inline(always)]
fn get_value(split: Vec<&str>) -> String {
    split[1..].join(" ").to_string()
}

#[inline(always)]
fn strip_line(line: &String) -> String {
    line.replace(&['[', ']', '"'][..], "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn it_strips_correctly() {
        let target = strip_line(&"[White \"Fischer, Robert J.\"]".to_string());
        let right_output = "White Fischer, Robert J.";
        assert_eq!(target, right_output);
    }

    #[test]
    fn it_gets_the_value_correctly() {
        let target = strip_line(&"[White \"Fischer, Robert J.\"]".to_string());
        let right_output = "White Fischer, Robert J.";
        assert_eq!(target, right_output);
    }

    #[test]
    fn test_new_line_parser_for_multiple_pgns() {
        let test_file = format!(
            "{}/test/pgns.pgn",
            std::env::current_dir().unwrap().to_str().unwrap()
        );

        let handle = match File::open(test_file) {
            Ok(s) => s,
            Err(e) => {
                panic!("{}", e)
            }
        };

        let mut iter = parse_file(handle);

        let first_pgn = iter.next().unwrap();
        let second_pgn = iter.next().unwrap();
        let third = iter.next();

        assert_eq!(first_pgn.white, "Robert James Fischer");
        assert_eq!(first_pgn.black, "Pal Benko");

        assert_eq!(second_pgn.white, "Fischer, Robert J.");
        assert_eq!(second_pgn.black, "Spassky, Boris V.");

        assert!(third.is_none());
    }
}
