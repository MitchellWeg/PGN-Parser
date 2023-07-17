use std::fs::File;
use std::io::BufReader;

use crate::parser::parse_lines;
use crate::pgn::PGN;

pub struct PGNIterator {
    pub total_size: u64,
    pub offset: u64,
    pub reader: BufReader<File>,
}

impl PGNIterator {
    pub fn new(file: File) -> PGNIterator {
        let total_size = file.metadata().unwrap().len();
        let reader = BufReader::new(file);

        PGNIterator {
            reader,
            total_size,
            offset: 0,
        }
    }
}

impl Iterator for PGNIterator {
    type Item = PGN;

    fn next(&mut self) -> Option<Self::Item> {
        let (new_offset, pgn) = parse_lines(&mut self.reader, self.offset);

        self.offset = new_offset;

        pgn
    }
}
