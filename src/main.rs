use csv::WriterBuilder;
use std::fs::File;
use std::io;

mod parser;
mod pgn;
mod pgn_iterator;
mod progressbar;

use progressbar::CustomProgressBar;

fn main() {
    let input: String = std::env::args().nth(1).expect("filename was not specified");
    let output: String = std::env::args()
        .nth(2)
        .expect("output file was not specified");

    let handle = match open_file(input) {
        Ok(h) => h,
        Err(e) => {
            print!("unable to open file due to {:?}", e);
            return;
        }
    };

    let mut iter = parser::parse_file(handle);

    write_to_csv(&mut iter, &output);
}

fn write_to_csv(iter: &mut pgn_iterator::PGNIterator, output_file: &str) {
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_path(output_file)
        .unwrap();

    let headers = ["white", "black", "game_result", "moves"];
    let pb = CustomProgressBar::new(iter.total_size);

    match writer.write_record(&headers) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }

    while let Some(pgn) = iter.next() {
        let data = [pgn.white, pgn.black, pgn.game_result, pgn.moves];

        pb.update(iter.offset);

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
