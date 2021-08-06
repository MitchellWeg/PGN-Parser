use std::env;
use std::io;
use std::fs::File;
use csv::Writer;

mod parser;
mod file;

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

    let output_handle = match File::create(output) {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };

    let out = parser::parse_file(handle);

    let data = serialize_to_format(out);

    file::write_to_file(output_handle, data);
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

pub fn serialize_to_format(input: Vec<parser::PGN>) -> String {
    let mut writer = Writer::from_writer(vec![]);

    for pgn in input {
         match writer.serialize(parser::PGN {
            event: pgn.event,
            site: pgn.site,
            date: pgn.date,
            round: pgn.round,
            white: pgn.white,
            black: pgn.black,
            game_result: pgn.game_result,
            white_elo: pgn.white_elo,
            black_elo: pgn.black_elo,
            time_control: pgn.time_control,
            termination: pgn.termination,
            moves: pgn.moves
        }) {
            Ok(_) => (),
            Err(e) => panic!(e)
        };

    };

    return String::from_utf8(writer.into_inner().unwrap()).unwrap();
}


