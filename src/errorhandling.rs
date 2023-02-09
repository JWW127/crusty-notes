#![allow(dead_code)]
#![allow(unused_parens, unused_must_use, unused_variables)]

use std::fs;
use std::io::{self, Read};

/*
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
▓Rust can be split into two types of errors▓
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
#1 Recoverable
- errors that can be tried again, (file not found, input validation, etc)
- can use Result<T,E>

#2 Non-Recoverable
- product of bugs, (out of range)
- can use panic! to stop execution
*/

pub fn err_main() {
    // This file will explore idiomatic ways to handle errors

    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //▓            Ignore Err                    ▓
    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //Sometimes its can be beneficial to just ignore the error

    fn ignore() {
        let data = std::fs::read_to_string("./path_we_know_works").unwrap();
        println!("{}", data);
        //as usual unwrap is unsafe, but if our data doesnt exist
        //we will kinda get what we want anyway a panic
    }

    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //▓            Terminate Err                 ▓
    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //If a error cant be handled we can terminate and provide error message
    fn terminate_give_error() {
        let data = fs::read_to_string("./path_we_know_works").expect("no data file");
        println!("{}", data);
        //read_to_string returns a result<String, Err> if we get an Err our
        // .expect method will log the provided value "no data file"
    }

    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //▓           falllback unwrap_or            ▓
    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //We can use unwrap_or to provide a default fallback value
    fn fallback_with_unwrap_or() {
        let will_err = None;
        let will_fallback = will_err.unwrap_or("Im your default fallback".to_string());
        println!("{}", will_fallback);
    }

    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //▓             Adding Abort                 ▓
    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //Aborting will immediately end program without cleanup
    //You can set panic to abort in your cargo.toml
    // [profile.release]
    // panic = 'abort'

    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //▓             Using Panic                  ▓
    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    fn panic_basics() {
        panic!(" Catastrophic Failure ")
        // this will stop the program print and print the text
        // you will all get a backtrace
    }

    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //▓          Panic Non existent Files        ▓
    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    // We can use match + file create + panic to satisfy our error
    fn no_file_found() {
        let my_file_result = std::fs::File::open("i-dont-exist.txt");

        let check_file_exist = match my_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                //⚠️ If we match a NotFound, do a match on a file creation,
                //  if it returns Ok return the created file
                //  else panic
                std::io::ErrorKind::NotFound => match std::fs::File::create("create-me.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Error creating file {:?}", e),
                },
                other_errors => {
                    panic!("Problem accessing file {:?}", other_errors);
                }
            },
        };
    }
    //★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★
    fn no_file_found_closure() {
        let my_file_res = std::fs::File::open("nope.txt").unwrap_or_else(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                std::fs::File::create("yup.txt").unwrap_or_else(|error| {
                    panic!("Error creating file {:?}", error);
                })
            } else {
                panic!("Problem accessing the file {:?}", error);
            }
        });
    }

    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //▓          Propagate Errors                ▓
    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    fn skeleton_propagate() -> Result<String, io::Error> {
        // the main idea is that the error is returned and handled by calling fn

        //🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀
        let my_file = std::fs::File::open("doesnt_exist.txt");

        // file exist store in 'get_file' else return Err
        let mut get_file = match my_file {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        //🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀
        //
        let mut val = String::new();

        //🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑
        // call read to string on 'get_file' store result in val if Ok
        // else return Err
        match get_file.read_to_string(&mut val) {
            Ok(_) => Ok(val),
            Err(e) => Err(e),
        }
        //🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑🦑
        // now its up to calling fn to decide what to do with return
        // depending on info it can panic or use default value
    }

    fn shorthand_propagate() -> Result<String, io::Error> {
        let mut val = String::new();
        // following line up to '?' replaces 🦀code block above
        // everything after the '?' replaces 🦑code block above
        std::fs::File::open("doesnt_exist.txt")?.read_to_string(&mut val);
        Ok(val)
    }
}
