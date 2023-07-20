use std::fs::File;

use crate::pgn_iterator::PGNIterator;

pub struct PGNHandler {
    pub total_size: u64,
    pub thread_count: i8,
    pub chunks: Vec<PGNIterator>,
}

impl PGNHandler {
    pub fn new(file: File, thread_count: i8) -> PGNHandler {
        let total_size: u64 = file.metadata().unwrap().len();

        let chunks = PGNHandler::create_chunks(file, total_size);

        PGNHandler {
            total_size,
            thread_count,
            chunks,
        }
    }

    pub fn create_chunks(file: File, total_size: u64) -> Vec<PGNIterator> {
        let mut iters: Vec<PGNIterator> = Vec::new();

        let iter = PGNIterator::new(file, 0, total_size);

        iters.push(iter);

        iters
    }
}
