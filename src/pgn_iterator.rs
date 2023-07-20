use std::fs::File;
use std::io::BufReader;

use crate::parser::parse_lines;
use crate::pgn::PGN;

pub struct PGNIterator {
    pub total_size: u64,
    pub min_offset: u64,
    pub max_offset: u64,
    pub reader: BufReader<File>,
}

impl PGNIterator {
    pub fn new(file: File) -> PGNIterator {
        let total_size = file.metadata().unwrap().len();
        let reader = BufReader::new(file);

        PGNIterator {
            reader,
            total_size,
            min_offset: 0,
            max_offset: total_size,
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
