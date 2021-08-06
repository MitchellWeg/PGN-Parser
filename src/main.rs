use std::env;
use std::fmt::Error;
use std::io;
use std::fs::File;
use csv::Writer;

mod parser;
mod file;

enum Format {
    csv,
    json
}

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

    let data = match serialize_to_format(Format::json, out) {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };

    match file::write_to_file(output_handle, data) {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
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

fn serialize_to_format(format: Format, input: Vec<parser::PGN>) -> Result<String, csv::Error> {
   match format {
       Format::csv => { 
           match write_as_csv(input){ 
               Ok(s) => return Ok(s),
               Err(e) => return Err(e)
           }
       },
       Format::json => Ok(write_as_json(input).unwrap())
   }
}

fn write_as_csv(input: Vec<parser::PGN>) -> Result<String, csv::Error> {
    let mut writer = Writer::from_writer(vec![]);

    for pgn in input {
         match writer.serialize(parser::PGN {
            event: pgn.event,
            site: pgn.site,
            date: pgn.date,
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
            Err(e) => return Err(e)
        };

    };

    Ok(String::from_utf8(writer.into_inner().unwrap()).unwrap())
}
fn write_as_json(input: Vec<parser::PGN>) -> Result<String, csv::Error> {
    let mut output: String = String::new();

    for pgn in input {
        let j = serde_json::to_string(&pgn);

        output.push_str(&j.unwrap());
        output.push_str(&"\r\n".to_string());
    }

    return Ok(output);
}
