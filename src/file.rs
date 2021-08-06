use std::io::prelude::*;
use std::io;
use std::fs::File;

pub fn write_to_file(mut file: File, data: String) -> Result<(), io::Error> {
    match file.write_all(data.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(e)
    }

    println!("done!");

    Ok(())
}
