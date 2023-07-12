use serde::Serialize;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct PGN {
    pub data: HashMap<String, String>,
}

impl Default for PGN {
    fn default() -> PGN {
        PGN {
            data: HashMap::new(),
        }
    }
}

pub struct PGNIterator {
    offset: Option<u64>,
    reader: BufReader<File>,
}

impl PGNIterator {
    pub fn new(file: File) -> PGNIterator {
        let reader = BufReader::new(file);

        PGNIterator {
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

                if pgn_line.starts_with("1.") {
                    pgn.data.insert("Moves".into(), get_value(split));
                    continue;
                }

                pgn.data.insert(split[0].into(), get_value(split));
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

        assert_eq!(first_pgn.data["White"], "Robert James Fischer");
        assert_eq!(first_pgn.data["Black"], "Pal Benko");

        assert_eq!(second_pgn.data["White"], "Fischer, Robert J.");
        assert_eq!(second_pgn.data["Black"], "Spassky, Boris V.");

        assert!(third.is_none());
    }
}
