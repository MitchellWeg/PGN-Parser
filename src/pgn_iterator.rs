use std::fs::File;
use std::io::BufReader;

use crate::parser::parse_lines;
use crate::pgn::PGN;

pub struct PGNIterator {
    pub min_offset: u64,
    pub max_offset: u64,
    pub reader: BufReader<File>,
}

impl PGNIterator {
    pub fn new(file: File, min_offset: u64, max_offset: u64) -> PGNIterator {
        let reader = BufReader::new(file);

        PGNIterator {
            reader,
            min_offset,
            max_offset,
        }
    }
}

impl Iterator for PGNIterator {
    type Item = PGN;

    fn next(&mut self) -> Option<Self::Item> {
        let (new_offset, pgn) = parse_lines(&mut self.reader, self.min_offset, self.max_offset);

        self.min_offset = new_offset;

        pgn
    }
}
