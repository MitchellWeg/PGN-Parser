use csv::WriterBuilder;
use std::fs::File;
use std::io;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::{cmp::min, fmt::Write};

mod parser;

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

fn write_to_csv(iter: &mut parser::PGNIterator, output_file: &str) {
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_path(output_file)
        .unwrap();

    let headers = ["white", "black", "game_result", "moves"];
    let pb = construct_progressbar(iter.total_size);

    match writer.write_record(&headers) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }

    while let Some(pgn) = iter.next() {
        let new = min(iter.offset, iter.total_size);
        pb.set_position(new);

        let data = [pgn.white, pgn.black, pgn.game_result, pgn.moves];

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

fn construct_progressbar(total_size: u64) -> ProgressBar {
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    pb
}
