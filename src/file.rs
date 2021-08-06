use std::io::prelude::*;
use std::fs::File;

pub fn write_to_file(mut file: File, data: String) {

    match file.write_all(data.as_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
    }

}
