#![allow(dead_code)]
#![allow(unused_parens, unused_must_use, unused_variables)]
use colored::*;
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

    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    //▓          Understanding '?'               ▓
    //▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    fn understanding_question_mark() {
        fn return_ok_int(x: u32) -> Result<u32, String> {
            if x != 0 {
                Ok(x)
            } else {
                Err("not a u32".to_string())
            }
        }

        let res_42 = return_ok_int(42);
        let res_match_check = match res_42 {
            Ok(val) => val,
            Err(_) => 0,
        };

        // or we can just shortcut it with a '?' like this
        fn res_question_mark(num: u32) -> Result<u32, String> {
            let check = return_ok_int(num)?;
            Ok(check)
        }
    }

    #[derive(Debug)]
    enum MyError {
        Bad(()),
    }
    //-------------------------------------------------------------------------
    //🍟🍟ERROR HANDLING WITH Results and CUSTOM ERRORS 🍟🍟
    fn get32(n: i32) -> Result<i32, MyError> {
        match n != 0 {
            true => Ok(n),
            false => Err(MyError::Bad(println!("{}", "bad".bright_red()))),
        }
    }

    fn is_res(i: i32) -> Result<i32, MyError> {
        let res = get32(i);
        res
    }

    fn custom_errors_results() {
        let cool = is_res(0);
        println!("{:?}", cool.unwrap())
    }
    //-------------------------------------------------------------------------
    //🍟🍟ERROR HANDLING WITH OPTIONS and CUSTOM ERRORS 🍟🍟
    fn get_num(n: i32) -> Option<i32> {
        if n != 0 {
            Some(n)
        } else {
            None
        }
    }

    fn my_res(i: i32) -> Result<i32, MyError> {
        let res = get_num(i).ok_or(MyError::Bad(println!("something {} happened", "bad".red())));
        res
    }

    fn custom_errors_opt() {
        let cool = is_res(0);
        println!("{:?}", cool.unwrap())
    }
    //-------------------------------------------------------------------------
    // How we might handle status codes errors
    use reqwest::header::HeaderMap;
    use reqwest::StatusCode;
    use std::error::Error;

    pub async fn get_data(api_key: &str) -> Result<String, Box<dyn Error>> {
        let url = "https://some.api.com/v1/user/post";
        let secret = format!("Bearer {}", api_key);

        // creates request headers
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", secret.as_str().parse()?);
        headers.insert("Accept", "application/json".parse()?);

        // client builder, we can use this make the actual request once we have headers
        let client = reqwest::Client::new();
        let response = client.get(url).headers(headers).send().await?;

        // https responses have status codes
        let status = &response.status();

        // we check if status code is 200 OK
        if status != &StatusCode::OK {
            // if not we can return our dyn Error like so
            Err(format!("{}", status))?;
        };

        Ok(response.json().await?)
    }
}
