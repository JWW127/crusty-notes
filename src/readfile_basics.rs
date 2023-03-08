use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn readfile_basics() {
    // creates file object that requires a path
    // instead of handling error we will just crash with unwrap for now
    let f = File::open("readme.md").unwrap();
    // create a new buf reader, takes a file
    let mut reader = BufReader::new(f);

    //
    let mut line = String::new();

    loop {
        // read bytes until newline append data to buf'&mut line', save len to len
        let len = reader.read_line(&mut line).unwrap();

        // if len is empty exit loop
        if len == 0 {
            break;
        }

        // if len not empty print the
        println!("{} ({} bytes long)", line, len);

        line.truncate(0);
    }
}
