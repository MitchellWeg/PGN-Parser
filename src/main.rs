use csv::{Writer, WriterBuilder};
use pgn_iterator::PGNIterator;
use std::fs::File;
use std::io;

mod parser;
mod pgn;
mod pgn_handler;
mod pgn_iterator;
mod progressbar;

use progressbar::CustomProgressBar;

fn main() {
    let input: String = std::env::args().nth(1).expect("filename was not specified");
    let output: String = std::env::args()
        .nth(2)
        .expect("output file was not specified");

    let tc: String = std::env::args()
        .nth(3)
        .expect("thread count was not specified");

    let handle = match open_file(input) {
        Ok(h) => h,
        Err(e) => {
            print!("unable to open file due to {:?}", e);
            return;
        }
    };

    let thread_count: i8 = match tc.parse() {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    };

    let mut iter = parser::parse_file(handle, thread_count);

    write_to_csv(&mut iter, &output);
}

fn write_to_csv(pgn_handler: &mut pgn_handler::PGNHandler, output_file: &str) {
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_path(output_file)
        .unwrap();

    let mut iter: pgn_iterator::PGNIterator = pgn_handler.chunks.pop().unwrap();

    handle_pgn(&mut iter, &mut writer);
}

fn handle_pgn(iter: &mut PGNIterator, writer: &mut Writer<File>) {
    let headers = ["white", "black", "game_result", "moves"];

    // TODO: each thread will probably get it's own progress bar,
    // so this will probably change to max_offset
    let pb = CustomProgressBar::new(iter.max_offset);

    match writer.write_record(&headers) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }

    while let Some(pgn) = iter.next() {
        let data = [pgn.white, pgn.black, pgn.game_result, pgn.moves];

        pb.update(iter.min_offset);

        match writer.write_record(&data) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
    }
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
