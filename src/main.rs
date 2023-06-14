#![allow(dead_code)]
#![allow(unused_parens, unused_must_use, unused_variables)]
use colored::*;
use std::error::Error;
mod closure_strategies;
mod errorhandling;
mod mandelbrot_ria;
mod readfile_basics;
mod testing_basics;
use num::complex::Complex;
use rusqlite::{Connection, Result};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use strum_macros::EnumString;

//single line comment ⚠️ ✅❌❗☠️ 🥰
/* multi line comment */
/// docs comment
// u8 ---> 8bits
// usize ---> 64bits
// char ---> 4 bytes, largest utf8 ---> 4 bytes
// {:#?} pretty format for collections
// '_' == All/NA
// '_name' == silence warnings will use later
//use std::str::SplitWhitespace;

fn main() {
    or_else_with_options()
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn casting_with_as() {
    let my_number = 100;
    println!("{}", my_number);
    // cast as a u8
    println!("{}", my_number as u8);
    // double cast
    println!("{}", my_number as u8 as char);
    // double casting not recommend instead explicitly cast type
    let my_u8: u8 = 100;
    println!("{}", my_u8 as char);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn len_is_bytes() {
    // ⚠️ len is number of bytes for a char not number of elements
    // we can prove this by printing len of 2 differently byte sized chars
    println!("just a single char with a size of {} byte", "z".len()); // 1 byte
    println!("just a single char with a size of {} byte", "ʐ".len()); // 2 bytes
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn len_vs_count() {
    let slice = "ʐebra";
    println!("len of ʐebra is {} bytes", slice.len());
    println!("count of ʐebra is {}", slice.chars().count());

    // to get the count dont forget to convert into chars
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn shadowing_blocks_it_doesnt_destroy() {
    let my_num = 8;
    println!("{}", my_num); // 8

    {
        let my_num = 69;
        println!("{}", my_num); //69
    }

    println!("{}", my_num); // 8
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn comparing_refs_n_derefs() {
    let my_27 = 27;
    let my_ref_27 = &my_27;
    let my_42 = 42;
    let my_ref_42 = &my_42;
    let my_double_ref_42 = &my_ref_42;

    // use * as a counter spell and deref
    println!("{}", my_27 == *my_ref_27);
    // use more ** to counter more refs
    println!("{}", my_42 == **my_double_ref_42);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn printing_n_escaping() {
    println!("regular pring");
    println!("\t with a tab");
    println!("\n with a newline");
    println!(
        "you can print
on seperate lines"
    );
    println!(r#"to print everything as it is typed \t \n"#);
    println!("escape brackets with brackets {{}}");
    println!("get char bytes with 'b' {:#?}", b"get my bytes");
    println!("print as hex {:x}", 'H' as u32);
    println!("print unicode \u{5C45}");

    let noice = 69;
    let noice_ref = &noice;
    let fname = "james";
    let lname = "bond";

    println!("get mem address of a ref {:p}", noice_ref);
    println!("change print order {1}, {0}", "zero", "one");
    println!("name {last} {first} {last}", first = fname, last = lname);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn string_vs_str() {
    // create a new string
    let imma_string = String::from("super mario bros");
    // create a slice
    let imma_slice = "donkey kong";
    // create string from a slice
    let imma_string_from_a_slice = "king Koopa".to_string();
    // create a string from a slice variable
    let imma_string_from_a_var = String::from(imma_slice);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn printing_mem_sizes() {
    // get the size of a type
    println!("strings are {:?}", std::mem::size_of::<String>());
    println!("i8 are {:?}", std::mem::size_of::<i8>());
    println!("f64 are {:?}", std::mem::size_of::<f64>());
    // get the mem size of a val
    println!("hello is {:?} bytes", std::mem::size_of_val("hello"));
    // get size of mem variable
    let goomba = "goomba";
    println!("my varible is {:?} bytes", goomba);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn format_vs_print() {
    let fname = "victra";
    let lname = "julii";
    let age = 33;
    let active = true;

    // format basically converts multiple types into a string for use
    let online_status = format!("{} {} age {} online status: {}", fname, lname, age, active);
    println!("{}", online_status);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn using_const() {
    // const must have types. they are immutable, cannot be shadowed.
    // commonly used for things that never change 'days of the week' '100%'
    // 'Static' is similar to const but with a permanent address
    const MY_CONST_IS_ALL_CAPS: u32 = 42;
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn ownership_practice() {
    // car is current owner
    let car = String::from("Honda");
    // car still current owner
    let ref_a = &car;
    // car still owner
    let ref_b = &car;
    // car 'honda' is now blocked via shadowing
    let car = String::from("Nissan");
    println!("{}", car); //Nissan
                         // But we can still access 'Honda' via our refs
    println!("ref_a{} ref_b{}", ref_a, ref_b); //Honda Honda
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn mutable_practice() {
    // RULE 1 : Only one is allowed per context
    // RULE 2 : Origin variable must be mutable
    // RULE 3 : Cannot be both mutable and immutable (impossible logic)

    let mut my_mutable_var = 42;
    let mutable_ref = &mut my_mutable_var;
    // using a deref on our mutable_ref we can then change our original var 'my_mutable_var'
    *mutable_ref = 777;

    println!(
        "ownership is still the same but we did change its val to {}",
        my_mutable_var
    );
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn passing_mutable_refs(my_mut_ref: &mut String) {
    // when calling this fn types must match
    println!("{}", my_mut_ref);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn copy_n_clone() {
    println!("copy creates a shallow copy of a target");
    println!("clone creates a deep copy of target");
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn vecs() {
    // vecs are very similar to arrays in JS, they can grow and shrink
    // initialize an empty vector
    let mut my_vec = Vec::new();
    my_vec.push("mario");
    my_vec.push("luigi");
    println!("{:?}", my_vec); // ["mario", "luigi"]

    // vecs assign a capacity based on contents, increments of 4
    println!("{}", my_vec.capacity()); // 4

    // ⚠️ capacity jumps to 8 when we have 5 items
    my_vec.push("peach");
    my_vec.push("bowser");
    my_vec.push("yoshi");
    println!("{}", my_vec.capacity()); // 8
                                       // assign your own capacity
    let mut custom_vec_cap = Vec::with_capacity(32);
    custom_vec_cap.push(42);
    custom_vec_cap.push(27);
    println!("{}", custom_vec_cap.capacity()); // 32

    // get a slice of a vec
    let slice_of_a_vec = &my_vec[1..3];
    println!("{:?}", slice_of_a_vec); // ["luigi", "peach"]

    // vec shorthand macro
    let vec_mac = vec!["toad", "goomba"];
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn tuples() {
    // create a tuple with ()
    let my_tuple = (
        42,
        "im a slice",
        [8, 9, 10],
        String::from("turn me into a string"),
    );

    // uses debug print
    println!("{:?}", my_tuple);
    // can access specific index with dot notation
    println!("{:?}", my_tuple.3);
    // tuples can be destructured
    let (a, b, c, ..) = my_tuple;
    println!("{}{}{:?}", a, b, c); // 42, "im a slice", [8,9,10]
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn matching(num: u8, first: &str, last: &str, w: i32, h: i32) {
    // matches must account for all possibilites '_' is a catchall
    let num_match = match (num) {
        1 => format!("your number is 1"),
        26 => format!("your number is 27"),
        42 => format!("your number is 42"),
        _ => format!("huh?"),
    };
    println!("{}", num_match);

    // Tuple Matchin
    let name_match = match (first, last) {
        ("tony", "stark") => format!("hello {} {} aka IronMan", first, last),
        ("bruce", "wayne") => format!("hello {} {} aka Batman", first, last),
        _ => format!("who are you?"),
    };
    println!("{}", name_match);

    // use '@' for a more fine grain match
    let rectangles_and_squares = match (w, h) {
        // reads like "when width is 0, whatever height is we cant calc the area"
        (w @ 0, h @ _) => format!("width is 0 this has no area"),
        (w @ _, h @ 0) => format!("height is 0 this has no area"),
        // you can do conditionals before the return
        (x, y) if x != y => format!("this is a rectangle its area: {}", x * y),
        (x, y) if x == y => format!("this is a square its area: {}", x * y),
        _ => format!("something unusual happened"),
    };
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn structs() {
    // a named tuple struct
    #[derive(Debug)] // we need debug to print
    struct RGB(u8, u8, u8);

    // a object stuct that contains our tuple struct from above
    #[derive(Debug)]
    struct Car {
        make: String,
        model: String,
        color: RGB,
    }

    let my_fav_color = RGB(115, 23, 2);
    let my_car = Car {
        make: String::from("Nissan"),
        model: String::from("R32"),
        color: my_fav_color,
    };

    println!("{:#?}", my_car);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn if_let_statement(a: bool) {
    let condition = a;
    //you can asign conditionals similar to ternary in js
    // var = condition ? true : false
    let my_num = if condition { 27 } else { 42 };
    // types must match if change { 42 } to { "42"} we get an error
    println!("{my_num}");
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
// we get an error because the ownership of the value of "name1" changes to name2
fn ownership_error() {
    let name1 = String::from("joseph");
    let name2 = name1; // ownership moved, uncomment next line to show error
                       // println!("{}", name1)
    println!("{}", name2);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn traits_n_types() {
    //traits kind of have a public methods feel to them
    // create a trait
    trait CheckIfNegativeOrPositive {
        fn is_negative(self) -> bool;
        fn is_positive(self) -> bool;
    }

    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //we can use traits on a type we made like a struct "our trait on our type"

    // create our struct
    struct Num {
        value: i32,
    }

    // now we can connect our trait with our struct
    impl CheckIfNegativeOrPositive for Num {
        // include the trait function to use
        fn is_negative(self) -> bool {
            // we want to check if negative
            self.value < 0
        }
        fn is_positive(self) -> bool {
            // we can also check if positive
            self.value > 0
        }
    }

    // instantiate our struct
    let my_num = Num { value: -42 };
    let my_num_2 = Num { value: -42 };
    // my_num implements our trait
    println!("{}", my_num.is_negative()); // true
    println!("{}", my_num_2.is_positive()); // false

    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //we can use traits on primatives, "Our trait on a foreign types"

    // since its not our type we dont have to make the type we just connect them
    impl CheckIfNegativeOrPositive for i32 {
        fn is_negative(self) -> bool {
            // we want to check if negative
            self < 0
        }
        fn is_positive(self) -> bool {
            // we can also check if positive
            self > 0
        }
    }

    println!("{}", 42.is_positive()); // true

    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    // we can also use a trait someone else made "foreign trait on our type"
    impl std::ops::Neg for Num {
        type Output = Num;

        fn neg(self) -> Num {
            Num { value: -self.value }
        }
    }

    let my_num_3 = Num { value: 5 };
    // long story short we use Neg from standard library to make negative
    let my_num_neg = -my_num_3;
    println!("{}", my_num_neg.value);

    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    /*
     * 3 ways to implement traits
     * 1. Your traits on Foreign Types
     * 2. Your Traits on Your Types
     * 3. Foreign Trait on Your Types
     */
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn implements_copy_trait() {
    // copy types implement copy trait, essentially avoiding ownership rules
    // copy types are so small there is no perf gain by not copying

    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    // here we make multiple copies of a i32
    let my_copy_type: i32 = 42;
    let copy_of_my_copy = my_copy_type;
    let copy_of_copy_of_my_copy = copy_of_my_copy;
    let another_copy_of_my_copy_type = my_copy_type;
    // yet we can still print all of them without any ownership errors
    println!("{}", my_copy_type);
    println!("{}", copy_of_my_copy);
    println!("{}", copy_of_copy_of_my_copy);
    println!("{}", another_copy_of_my_copy_type);
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    // this wont work because it is a struct
    struct Num {
        value: i32,
    }

    let my_num_struct = Num { value: 13 };

    let copy_of_not_a_copy_type = my_num_struct;
    // UNCOMMENT next line to see move ERROR because not a copy type
    //let another_copy_of_not_a_copy_type = my_num_struct;

    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    // we can make a struct copy trait work with a ref
    struct Bool {
        value: bool,
    }

    let my_bool_struct = Bool { value: true };
    let copy_of_ref_struct = &my_bool_struct; // works
    let another_copy_of_ref_struct = &my_bool_struct; //works
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn implement_traits_with_derive() {
    // we can use derive to implement traits on our structs
    #[derive(Clone, Copy)]
    struct Num {
        value: i32,
    }

    let my_num = Num { value: 42 };

    let copy_of_my_num = my_num;
    let another_copy = my_num; // would typically throw error here
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn generic_types<T: Display>(arg: T) {
    // generic fuction that has generic type 'T' that implements 'Display' trait
    // when a generic type implements a trait we call that a 'constraint'
    println!("{}", arg);
}
//◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
//◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
fn working_with_larger_generics() {
    // we need this to show how where can be used for generic traits
    #[derive(Debug)]
    struct Status {
        running: bool,
    }

    let chicago = Status { running: true };
    let austin = Status { running: true };
    let vegas = Status { running: false };
    let seattle = Status { running: true };

    // using 'where' to indicate traits for our generic types
    fn generic_types_with_where<T, U, V, W>(server_0: T, server_1: U, status_0: V, status_1: W)
    where
        T: Display,
        U: Display,
        V: Debug,
        W: Debug,
    {
        println!("## server: {} === ACTIVE: {:?} ##", server_0, status_0);
        println!("## server: {} === ACTIVE: {:?} ##", server_1, status_1);
    }

    // let give it a call
    generic_types_with_where(0, 1, chicago, austin);
    // let give it another call but with named servers and drill down our struct type
    generic_types_with_where("vegas", "seattle", vegas.running, seattle.running);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn using_min_max() {
    let get_smallest_number = std::cmp::min(1, 99); // 1
    let get_largest_number = std::cmp::max(1, 99); // 99
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn declare_methods(n: i32) {
    struct FavNumber {
        value: i32,
    }
    // this creates a method on our struct "FavNumber"
    // this is similar to traits but no trait just method
    impl FavNumber {
        // its probably better to use a ref when using self
        // as ownership is transferred and then dropped if not returned
        fn log_fav_number_plus_42(&self) -> i32 {
            self.value + 42
        }
    }

    let my_fav = FavNumber { value: n };
    println!("{}", my_fav.log_fav_number_plus_42()) // adds 42 to n
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn test_with_assert() {
    let answer = 42;
    //  eq stands for EQUAL. answer must equal 42
    assert_eq!(answer, 42);
    // answer must NOT EQUAL 41
    assert_ne!(answer, 41);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn for_in() {
    // this is a for loop, 3 is not included so 0-2
    for i in 0..3 {
        println!("count: {}", i);
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn array_iters() {
    let arr = [10, 20, 30];
    // using .iter() method more efficient than [..]
    for i in arr.iter() {
        println!("{}", i);
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn enum_basics(num_of_wheels: i32) {
    // create enum
    enum CarOrBike {
        Car,
        Bike,
    }
    // use match to help return the enum Car | enum Bike
    fn choose_vehicle(wheels: i32) -> CarOrBike {
        match wheels {
            0..=3 => CarOrBike::Bike,
            _ => CarOrBike::Car,
        }
    }
    // let check what enum was returned
    fn check_my_ride(enum_state: &CarOrBike) {
        match enum_state {
            CarOrBike::Bike => println!("Bikes are fun but can be dangerous"),
            CarOrBike::Car => println!("Cars are a bit safer with seat belts"),
        }
    }

    let n = num_of_wheels;
    let vehicle = choose_vehicle(n);
    check_my_ride(&vehicle);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn giving_enums_values() {
    #[derive(Debug)]
    enum ColorModel {
        Hex(u8),
        Rgb(u8, u8, u8),
    }

    let my_color = ColorModel::Rgb(42, 69, 255);
    println!("{:?}", my_color);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn importing_enums_using_strum(avenger: &str) {
    #[derive(Debug, PartialEq, EnumString)]
    enum Hero {
        Thor,
        Ironman,
        Spiderman,
        Hulk,
        BlackWidow,
        Hawkeye,
        CaptainAmerica,
        DoctorStrange,
        BlackPanther,
    }

    //see imported crate strum
    fn parse_avenger(input: &str) -> Hero {
        Hero::from_str(input).unwrap()
    }

    fn match_avenger(hero: &Hero) -> String {
        // no need to call Hero::Hero_Name everytime we import the hero part
        use Hero::*;
        let my_avenger = match hero {
            Thor => format!("Thor is cool"),
            Ironman => format!("Tony Stark"),
            Spiderman => format!("Peter Parker"),
            Hulk => format!("Bruce Banner"),
            BlackWidow => format!("Natasha"),
            Hawkeye => format!("Clint"),
            CaptainAmerica => format!("Steve Rogers"),
            DoctorStrange => format!("Strange"),
            BlackPanther => format!("chitala"),
        };
        my_avenger
    }

    let my_parsed_avenger_arg = parse_avenger(avenger);
    let my_matched_avenger = match_avenger(&my_parsed_avenger_arg);
    println!("{}", my_matched_avenger);

    /*
    There seems to be an possible bug with parse_avenger() as it can accept
    any &str despite the fact that it may not match with match_avengers()
    this will cause a panic. hmmmm, will have to review this in future.
    */
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

fn enums_have_ids() {
    // enums have ids that increment
    enum EnumId {
        A,
        B,
        C,
    }

    println!(
        "{:?},{:?},{:?}",
        EnumId::A as i32,
        EnumId::B as i8,
        EnumId::C as u32
    ); // 0,1,2

    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    // enums can have assigned id if they dont hold data
    enum EnumAssignedId {
        X = 10,
        Y = 20,
        Z = 42,
    }

    println!(
        "{:?},{:?},{:?}",
        EnumAssignedId::X as i32,
        EnumAssignedId::Y as i32,
        EnumAssignedId::Z as i32
    ); // 10,20,42

    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    // enum id use cases
    // we can create a vec from our enum
    let enum_vec = vec![EnumAssignedId::X, EnumAssignedId::Y, EnumAssignedId::Z];
    // loop over vec, for each enum match which returns print
    for vec_item in enum_vec {
        match vec_item as u32 {
            id if id == 10 => println!("your id is 10"),
            id if id == 20 => println!("your id is 20"),
            id if id == 42 => println!("the answer to the universe is 42"),
            _ => println!("your id isnt in our system"),
        }
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn methods_structs_usage() {
    #[derive(Debug)]
    struct Rect {
        height: i32,
        width: i32,
    }

    impl Rect {
        fn get_area(&self) -> i32 {
            self.height * self.width
        }
        fn get_perimeter(&self) -> i32 {
            (self.height + self.width) * 2
        }
        fn change_values(&mut self, height: i32, width: i32) {
            self.width = width;
            self.height = height;
        }
    }

    let mut my_rect = Rect {
        height: 5,
        width: 5,
    };

    // change vals to 10
    my_rect.change_values(10, 10);

    println!("{:?}", my_rect.get_area()); //100
    println!("{:?}", my_rect.get_perimeter()); //40
    dbg!(my_rect);
    //dbg is for quick debugging basically pretty prints. be aware it also takes ownership
    //moving dbg! above our prints will cause a mover error
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn associated_struct_creation_methods() {
    // unique methods for creating instances of self
    struct Fruit {
        name: String,
    }

    impl Fruit {
        // creating our associated function
        fn new(fruit: String) -> Self {
            Self { name: fruit }
        }
        //⚠️ the 'new' fn name is not a keyword just tradition/idiomatic
    }
    //⚠️ we have a different instatiation syntax
    let apple = Fruit::new(String::from("apple"));
    println!("{}", apple.name);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn naming_loops() {
    let mut counter = 0;
    let mut counter2 = 0;
    'outer_loop: loop {
        counter += 1;
        // once we hit 5 we enter inner_loop
        if counter > 5 {
            // prefix with '_' or typically we get a warning for unused label
            '_inner_loop: loop {
                counter2 += 1;
                if counter2 > 5 {
                    break 'outer_loop;
                }
            }
        }
    }
    println!("Dobbie is free");
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn return_val_from_loop() {
    let mut counter = 0;
    let my_loop_return = loop {
        counter += 1;
        if counter > 5 {
            //break & desired return value -> both exit loop and return val
            break String::from("im a val returned from a loop");
        }
    };
    println!("{}", my_loop_return);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn using_continue() {
    // we can use the 'continue' keyword to give access to later code
    // I like to think of it similar to a IF Else
    for num in 1..=10 {
        if num % 2 == 0 {
            continue;
        }
        println!("even number"); //prints on even num
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn using_a_while_loop() {
    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("{}", counter);
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn loop_range_tricks() {
    //creating a demo vec of chars
    let my_vec = vec!['a', 'b', 'c', 'd'];
    // loop over a ref of our vec
    for item in &my_vec {
        // print each item
        println!("{}", item);
    }
    // loop over number range non-inclusive of 3
    for item in 0..3 {
        println!("{}", item) // 0,1,2
    }
    // loop over number range including 3
    for item in 0..=3 {
        println!("{}", item) // 0,1,2,3
    }
    // loop over unused item
    for _ in &my_vec {
        println!("looping...") // looping
    }

    /*
    main syntax 'for in' loops
    for (item) in (range) { do something }

    main syntax 'while' loops
    while (condition is true) { do something }

    main syntax for pure 'loops'
    'loop_name: loop { do something until 'break' keyword is hit}
    */
}

fn extending_iters() {
    // create our first iterable type, mut vec
    let mut v1 = vec![1, 2, 3];
    // we can extend(add to) with like collection types
    let v2 = vec![4, 5, 6];
    // this is an array but the contained types match
    let a1 = [42, 69];

    v1.extend(v2.iter());
    v1.extend(a1.iter());
    println!("{:?}", v2); // 1,2,3,4,5,6,42,69
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn rusqlite_usage() -> Result<()> {
    // quick and dirty use of rusqlite to connect to sqlite database
    // insert our struct user data

    // we'll need a struct for this example
    #[derive(Debug)]
    struct User {
        id: i32,
        name: String,
        data: Option<Vec<u8>>,
    }

    // instantiate our struct with our data 'me'
    let me = User {
        id: 0,
        name: "Joseph".to_string(),
        data: None,
    };

    // create our db connection, if no db create one at provided path
    let conn = Connection::open("./test.db")?;

    // open connection create a table called 'user' with a id,name,data
    conn.execute(
        "CREATE TABLE user (
            id INTEGER PRIMARY KEY, 
            name TEXT NOT NULL,
            data BLOB
        )",
        (),
    )?;

    // open connection Insert our 'me' values into our previously created table
    conn.execute(
        "INSERT INTO user (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    // prepare a sql query statement that targets our database/table values of id, name, data
    let mut stmt = conn.prepare("SELECT id, name, data FROM user")?;
    // use our prepared statement to map over our db data and create a iterable
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    // iterate over iterable we created above and print the values
    for user in user_iter {
        println!("{:?}", user.unwrap());
    }
    Ok(())
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓NOTES▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
// How we might like to think about Options and Results
// Options - have something or nothing
// Results - have an OK or a Err, it will work or will return an error
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn options_some_none() {
    // first lets make a fn that returns an option type
    fn get_fav_theme(theme: &str) -> Option<&str> {
        // our fn takes a &str and returns a Some(&str) or none
        match theme {
            "light" => Some("light theme applied"),
            "dark" => Some("dark theme applied"),
            _ => None,
        }
    }

    // now we can use fn that has a Option returned in Options match
    println!(
        "{}",
        match get_fav_theme("dark") {
            Some(x) => x,
            None => "Use default theme",
        }
    )
}

fn options_if_else() {
    fn gt_42(val: i32) -> Option<i32> {
        if val < 42 {
            None
        } else {
            Some(val)
        }
    }

    let my_num = 69;
    let my_num2 = 7;
    let my_num3 = 1;
    println!("{:?}", gt_42(my_num).unwrap()); //unwrap panics on none, thats bad
                                              //
    println!("{:?}", gt_42(my_num2).expect("needs to be greater than 42")); //better than unwrap

    // even better is to use a match to handle errors
    match gt_42(my_num3) {
        Some(num) => println!("{} is greater than 42", num),
        None => println!("error number needs to be greater than 42"),
    }
    /*
    other ways to unwrap safely
        '.is_some()' method checks if attached variable is a 'some' value returns bool
        '.unwrap_or(default)' if none value returns default esle returns some value
    */
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn error_checking_with_results() {
    // you must have a fn that returns Result enum values Ok & Err
    fn return_ok_int(x: u32) -> Result<u32, String> {
        if x != 0 {
            Ok(x)
        } else {
            Err("not a u32".to_string())
        }
    }
    // lets make a call to it and save the result 4 times. 3xOk 1xErr
    let res_42 = return_ok_int(42);
    let res_69 = return_ok_int(69);
    //let res_err = return_ok_int(3 - 4); // this will Err as it is -1
    // lets unwrap the first one, although unwrapping is unsafe
    println!("{}", res_42.unwrap()); //42 , but not 100% safe
                                     // we can be safer by treating our Result with a match
    let res_match_check = match res_69 {
        Ok(val) => val,
        Err(_) => 0,
    };
    // the added benefit of doing a match is Ok will unwrap without explicit call
    println!("{}", res_match_check); //69

    // this process is so common rust made a macro allowing you to skip the match
    fn return_ok_macro(x: u32) -> Result<u32, String> {
        let res_777 = return_ok_int(x)?;
        Ok(res_777) // this isnt unwrapped tho it is safe
    }
    println!("{:?}", return_ok_macro(777).unwrap())
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn destructuring_structs() {
    // standard struct creation
    struct User {
        id: u32,
        name: String,
        age: u32,
    }

    //standard struct instantiation
    let gandalf = User {
        id: 1,
        name: "Mithrandir".to_string(),
        age: 24000,
    };

    // instead of getting the data by dot notation we can just destucture
    // syntax: let STRUCT_NAME {field: alias, field2: alias, field3: alias} = variable
    let User {
        id: lotr_id,
        name: lotr_character,
        age: lotr_age,
    } = gandalf;

    println!("{}", lotr_character); //"Mithrandir"
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn result_is_ok_method(allow: bool) {
    fn returns_a_result(allow: bool) -> Result<(), ()> {
        if allow == true {
            Ok(())
        } else {
            Err(())
        }
    }

    let res = returns_a_result(allow);

    // using .is_ok method
    if res.is_ok() {
        println!("is_ok: ✔")
    } else {
        println!("is_ok: ✘ (err present)")
    }
    // using .is_err
    if res.is_err() {
        println!("is_err: ✔")
    } else {
        println!("is_err: ✘ (ok present)")
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn option_is_some_method(num: i32) {
    fn returns_a_opt(val: i32) -> Option<i32> {
        if val == 42 {
            Some(val)
        } else {
            None
        }
    }
    let opt = returns_a_opt(num);

    // using .is_some()
    if opt.is_some() {
        println!("is_opt: ✔")
    }
    // using .is_none()
    if opt.is_none() {
        println!("is_none: ✘")
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn option_get_method() {
    let my_vec = vec!['a', 'b', 'c'];
    let get_idx_0 = my_vec.get(0);
    let get_idx_5 = my_vec.get(5); //note idx 5 doesnt exist

    // get will allow us to try to get idx 5 and not crash instead get a None
    println!("idx_0: {:?}, idx_5: {:?}", get_idx_0, get_idx_5);
    // idx_0: Some('a'), idx_5: None

    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    // we can use .get in a for in loop
    for idx in 0..10 {
        // if idx is in range assign to Some(num) and print
        // if its not in range it is a none and dont assign
        if let Some(num) = my_vec.get(idx) {
            println!("{}: in range", num);
        }
    }
    // prints out only the 'Some' values unwrapped a,b,c
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn slice_gets_derefs() {
    //create array
    let num_arr = [1, 2, 3, 4, 5];
    //create slice of entire array
    let num_slice = &num_arr[..];

    //what happens when we GET valid idx
    let valid_idx = num_slice.get(0);
    println!("valid.get: {:?}", valid_idx); // Some(1)
    let invalid_idx = num_slice.get(99);
    println!("invalid.get: {:?}", invalid_idx); // None

    // you can get value with unwrap
    let valid_unwrap = valid_idx.unwrap();
    println!("valid.get: {:?}", valid_unwrap); // 1

    let valid_idx_two = num_slice.get(1);
    // you can deref when unwrapped (see type dif)
    let valid_unwrap_deref = *valid_idx_two.unwrap();
    println!("valid deref: {:?}", valid_unwrap_deref); //2

    // unwrapping a "None" will cause a panic.

    let invalid_idx_two = num_slice.get(42);
    //  instead of panic unwarp_or
    let special_unwrap = invalid_idx_two.unwrap_or(&42);
    println!("special: {:?}", special_unwrap); //42
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn vec_into_iter_closure_collect() {
    let my_arr = vec![10, 20, 30, 40, 50];
    // read like, turn our vec into an iter, map over it, closing over every val
    // adding 5, then collect into a vec
    let my_closed_iter: Vec<i32> = my_arr.into_iter().map(|num| num + 5).collect();

    println!("{:?}", my_closed_iter); //[15, 25, 35, 45, 55]
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn chunks() {
    // create you have an arr or vec
    let my_vec = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
    // we just want a slice to work with
    let my_slice = &my_vec[1..];

    // we can loop our slice and chunk with for in loop
    for a_chunk in my_slice.chunks(2) {
        println!("{:?}", a_chunk);
    }
    // we can expect out put to be chunks of 2 &[i32] from our vec
    // [4,6]
    // [8,10]
    // [12,14]
    // [16,18]
    // [20] // note when theres is no more to chunk it will use whats left
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn vec_remove_sort_dedup() {
    //⚠️ we missing 4 and 5, also 42 shouldnt be there
    let mut my_vec_v1 = vec![1, 2, 3, 6, 42, 7, 8];
    my_vec_v1.insert(3, 4); //[1, 2, 3, 4, 6, 42, 7, 8];
    my_vec_v1.insert(4, 5); //[1, 2, 3, 4, 5, 6, 42, 7, 8];
    my_vec_v1.remove(6); //[1, 2, 3, 4, 5, 6, 7, 8];

    println!("{:?}", my_vec_v1);

    let mut v2 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v2.sort();
    v2.dedup(); // will extra 1s and 2s

    println!("{:?}", v2); // [1, 2, 5, 10, 11, 40];
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn box_the_smart_pointer() {
    // we can use box to add to heap and have pointer to it
    let b = Box::new(42);
    println!("{:p}", b);
    // 1. of the benefits of box is storing dynamic size types on heap
    // dsts have the special ?Sized bound
    // dsts - slices, trait objs, structs
    // 2. you can transfer ownership of boxed values
    //
    // Indicators we might want to use box
    // - have a large value you want to put on heap to save space on stack
    // - expiring lifetime that should be moved to another scope
    // - recursive data structure that would otherwise create a stack overflow
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn enum_match_game_example() {
    // different possible states of a game
    enum State {
        Start,
        Running { hp: u32 },
        GameOver(GoAnimation),
    }

    enum GoAnimation {
        Active,
        NotActive,
    }

    // our starting state
    let mut state = State::Start;
    // loop / match to update our state accordingly
    loop {
        match state {
            State::Start => {
                println!("Game Started");
                state = State::Running { hp: 100 } //what state will be after start
            }
            State::Running { hp: 0 } => {
                state = State::GameOver(GoAnimation::Active);
                println!("GG YOU'RE DEAD");
            }
            State::Running { ref mut hp } => {
                *hp -= 25;
                println!("OUCH TOOK 25 DMG");
            }
            State::GameOver(GoAnimation::Active) => {
                println!("Exiting Game");
                state = State::GameOver(GoAnimation::NotActive)
            }
            State::GameOver(GoAnimation::NotActive) => break,
        }
    }
    println!("Main Menu")
    //this is basically paraphrase of togglebits example
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn cloning_better_with_iter_map() {
    // data to work with
    let original_heros = vec!["ironman", "spiderman", "thor", "hulk"];
    // first we'll demonstrate a poor clone pattern
    let bad = original_heros
        .clone()
        .into_iter()
        .take(2)
        .collect::<Vec<_>>();
    // this is bad because we clone the whole vec for 2 elements
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //
    // now lets do a better version
    let better = original_heros
        .iter()
        .map(|i| i.clone())
        .take(2)
        .collect::<Vec<_>>();
    // this only clones what we need, but there is an even better version
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    //◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦
    // the above pattern is so common there is a builtin method 'cloned()'
    let best = original_heros.iter().cloned().take(2).collect::<Vec<_>>();
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn idx_vs_get() {
    //there are some important differences between vec[idx] and vec.get(idx)
    let a_vec = vec![1, 2, 3];
    let via_idx = &a_vec[0]; //type is &i32
    let via_get = &a_vec.get(0); //type &option<&i32>
                                 //get returns a option which we can then use for match some/none
    match via_get {
        Some(via_get) => println!("{via_get}"), //1
        None => println!("something is up"),
    }

    // this is important distinction because
    // vec[out of range] is a panic
    // vec.get(out of range) is a None
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn hashmap_basics() {
    /*
    ✅Are stored on the heap
    ✅Are varibale in size
    ⚠️ Copy types or those with the copy trait like i32 are copied into hashmap
      while owned types like strings will be move ownership
    */

    // we'll need a struct for demo

    //-------------------------------------------------------------------
    //      Simple user from person
    //-------------------------------------------------------------------
    struct Person {
        name: String,
        id: u8,
    }

    struct User {
        info: HashMap<u8, String>,
    }

    // create new person
    let bob = Person {
        name: String::from("Bob"),
        id: 42,
    };

    // create new user
    let mut user_bob = User {
        info: HashMap::new(),
    };

    // insert info
    user_bob.info.insert(bob.id, bob.name);
    //--------------------------------------------------------------------
    //--------------------------------------------------------------------

    #[derive(Debug)]
    struct City {
        name: String,
        population: HashMap<u32, u32>,
        retired: HashMap<u32, u32>,
        employed: HashMap<u8, u32>,
    }
    // instantiate
    let mut gotham = City {
        name: "Gotham".to_string(),
        population: HashMap::new(),
        retired: HashMap::new(),
        employed: HashMap::new(),
    };

    gotham.population.insert(2020, 200000);
    gotham.population.insert(2021, 300000);
    gotham.population.insert(2022, 400000);
    gotham.retired.insert(2020, 40000);
    gotham.retired.insert(2021, 42000);
    gotham.retired.insert(2022, 69000);

    // instead of hardcoding values we can use previously
    // entered values to calc our next insert
    // we use copied to convert Option<&u32> into a regular Option<u32>
    // allowing us to do our math arithmetic withouth needing to create a ref
    for (k, v) in gotham.population.iter() {
        let retired = gotham.retired.get(&k).copied().unwrap_or(0);
        let civ = v - retired;
        // entry = check if *k exist and has value, if it does do nothing
        // else create the *k and enter civ as value
        gotham.employed.entry(*k as u8).or_insert(civ); //each <k,v> = <u32,u32>
    }

    gotham.name = "Arkham".to_string();
    // Below will be unsorted, one way to sort is to use BTreeMap (essentially replace HashMap with
    // BTreeMap)
    println!("gotham: {:#?}", gotham);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn hashmap_to_track_word_count() {
    let text = "Fear is the mind-killer Fear is the little-death that brings total obliteration I will face my Fear I will permit it to pass over me and through me";
    let mut map = HashMap::new();

    // loop over text for every word check if it is a key with a value, if it is increment count, if not
    // use the word as a key and make 0 the value, then increment count
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn btreemap_basics() {
    // we'll need a struct for demo
    #[derive(Debug)]
    struct City {
        name: String,
        population: BTreeMap<u32, u32>,
        retired: BTreeMap<u32, u32>,
        employed: BTreeMap<u32, u32>,
    }

    let mut metropolis = City {
        name: "metropolis".to_string(),
        population: BTreeMap::new(),
        retired: BTreeMap::new(),
        employed: BTreeMap::new(),
    };

    metropolis.population.insert(2020, 200000);
    metropolis.population.insert(2021, 300000);
    metropolis.population.insert(2022, 400000);
    metropolis.retired.insert(2020, 40000);
    metropolis.retired.insert(2021, 42000);
    metropolis.retired.insert(2022, 69000);

    for (k, v) in metropolis.population.iter() {
        let retired = metropolis.retired.get(&k).copied().unwrap_or(0);
        let civ = v - retired;
        metropolis.employed.insert(*k, civ);
    }

    metropolis.name = "Metropolis".to_string();

    // ⚠️ this is more or less the same as the gotham example above except
    // our print is sorted
    println!("metropolis: {:#?}", metropolis);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn binary_heap_basics() {
    // the main benefit of binary heaps as we can predict the first return
    // as it will always be the largest
    let mut my_b_heap = std::collections::BinaryHeap::new();
    my_b_heap.push(42);
    my_b_heap.push(1);
    my_b_heap.push(3);
    my_b_heap.push(5);
    my_b_heap.push(7);
    my_b_heap.push(9);
    my_b_heap.push(11);
    while let Some(num) = my_b_heap.pop() {
        println!("heap 1: {:?}", num)
    }
    //★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★
    println!("\n");
    //★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★
    // another example to prove largest is first return
    let mut my_b_heap_2 = std::collections::BinaryHeap::new();
    my_b_heap_2.push(3);
    my_b_heap_2.push(6);
    my_b_heap_2.push(9);
    my_b_heap_2.push(43);
    my_b_heap_2.push(12);
    my_b_heap_2.push(15);
    my_b_heap_2.push(18);
    while let Some(num) = my_b_heap_2.pop() {
        println!("heap 2: {:?}", num)
    }
}
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn working_with_strings() {
    //strings can get weird

    /*⚠️ to append to a string you must add type (1:String, 2:&str)
     * what problems does this cause?
     * - by appending we give ownership to 1:String, so maybe we clone?
     * - what if we have to append multiple &str to the string?
     * - should we convert &str to 1:String to satisfy our needs?
     */

    // Example 1
    let mut hot_string = "hot".to_string(); //⚠️ mut, 'push_str' requires it
    let dog_slice = "dog";
    hot_string.push_str(dog_slice);
    let hotdog = hot_string;
    println!("{}", hotdog); //hotdog

    // Example 2
    let string_cheese = "cheese".to_string();
    let burger_slice = "burger";
    let cheeseburger = string_cheese + burger_slice;
    println!("{}", cheeseburger); //cheeseburger

    // Example 3
    let french_slice = "french";
    let fries_string = "fries".to_string();
    // ⚠️ you cant append to a &str french_slice must be a string and fries slice
    let frenchfries = french_slice.to_string() + &fries_string;
    println!("{}", frenchfries);

    // Example 4
    let peanut_string = "peanut".to_string();
    let butter_string = "butter".to_string();
    let jelly_string = "jelly".to_string();
    // ⚠️ we had to convert butter and jelly to &str
    let peanut_butter_jelly = peanut_string + &butter_string + "&" + &jelly_string;
    println!("{}", peanut_butter_jelly);

    // Example 5
    let kentucky_string = String::from("Kentucky");
    let fried_string = String::from("Fried");
    let chicken_string = String::from("Chicken");
    let kfc = format!("{kentucky_string} {fried_string} {chicken_string}");
    println!("{}", kfc); // Kentucky Fried Chicken

    //⚠️ rust does not provide idx string[idx] access of a string it must be a slice

    // Example 6
    let pizza = "pizza".to_string();
    // use slice notation to get specific characters in string
    let i = &pizza[1..2];
    println!("{}", i); //i

    // Example 7 looping over strings
    let spaghetti = "spaghetti".to_string();
    for c in spaghetti.chars() {
        println!("{c}"); // s,p,a,g,h,e,t,t,i
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn vecdeque_basics() {
    //Vecdeque is a specialized vec that excels at left shift from pop_front
    //VecDeque uses ring buffers to acheive significat speed over standard vecs
    let mut my_vec_deque = std::collections::VecDeque::from(vec![0, 690_000]);
    for i in 0..600000 {
        my_vec_deque.pop_front();
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn cli_args_basics() {
    // cargo run 'arg1' 'arg2'
    let first = std::env::args().nth(1).expect("supply an arg");
    let second = std::env::args().nth(2).expect("give 2 args");
    // cargo run '42' 'pizza'
    let n: i32 = first.parse().expect("arg should be an integer");
    let slice: String = second.parse().expect("2nd arg should be a string");
    println!("{} such a magical number", n); // 42 such a magical number
    println!("{} such a magical word", slice); // pizza such a magical word
}
//★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★
fn cli_args_match() {
    // cargo run 'arg: i32'
    let first = std::env::args().nth(1).expect("please enter a number");
    let n: i32 = first.parse().expect("must be a number");

    let text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "wtf",
    };
    println!("{}", text);
}
//★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★☆★
fn cli_args_loop() {
    //cargo run 'arg1' 'arg2' 'arg3' 'etc'
    for arg in std::env::args() {
        println!("{}", arg)
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn string_2_iter_2_enumerate() {
    // this example is from rust_in_action
    let penguin_data = "\
    common name,length (cm)
    Little Penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    // creates iterator over the lines of a &str
    let records = penguin_data.lines();

    // enumerate over an interator, i as idx and record as val
    for (i, record) in records.enumerate() {
        //continue if first row || continue if row is only white space
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        // for each line(record) split on ',' trim whitespace, add to collection
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim()) // take iter -> new iter
            .collect();

        // cfg checks config at compile time
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // field[1] should be a Vec<&str> try to parse into f32
        // if it works store it in variable 'length'
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn var_box_rc_arc_mutex() {
    //various ways to store an int

    // puts integer on stack
    let a = 10;
    //puts integer on heap "boxed integer"
    let b = Box::new(20);
    // "box integer inside a ref counter"
    let c = Rc::new(Box::new(30));
    // Int wrapped in atomic ref counter protected by mutex lock
    let d = Arc::new(Mutex::new(40));

    println!("a:{:?} b:{:?} c:{:?} d:{:?}", a, b, c, d);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn complex_math() {
    //a and b are both create a new Complex instance
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn matching_styles(x: i32) {
    match x {
        1000000 => println!("direct"),
        10..=40 => println!("range"),
        42 | 69 => println!("this or that"),
        _ => println!("wildcard"),
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn impl_with_dif_types() {
    //declare our struct
    struct MyNum<T> {
        x: T,
        y: T,
    }

    //declare if struct is i32
    impl MyNum<i32> {
        fn get_x(&self) -> &i32 {
            &self.x
        }
    }

    //declare if struct is &str
    impl MyNum<&str> {
        fn get_x(&self) -> String {
            self.x.to_string()
        }
    }

    // instantiate one of each
    let num = MyNum { x: 4, y: 5 };
    let str_num = MyNum {
        x: "four",
        y: "five",
    };

    // boom works as expected
    println!("{} = {}", str_num.get_x(), num.get_x())
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn common_iterator_usage() {
    let nums1 = vec![1, 2, 3, 4, 5];
    let nums2 = vec![1, 2, 3, 4, 5];
    let nums3 = vec![1, 2, 3, 4, 5];
    let nums4 = vec![1, 2, 3, 4, 5];
    let hw = vec!["hello", "world"];
    let abc = vec!['a', 'b', 'c'];

    // use map transform(adaptor)
    let squares: Vec<i32> = nums1.iter().map(|x| x * x).collect();
    println!("{:?}", squares);

    // use filter(adaptor) to filter when condition satisfied
    let odd_only: Vec<_> = nums2.iter().filter(|&x| x % 2 != 0).collect();
    println!("{:?}", odd_only);

    //fold(adaptor) into a single value
    let sums: i32 = nums3
        .iter()
        .fold(0, |accumulator, &current| accumulator + current);
    println!("{:?}", sums);

    // sum(consumer) works like fold but just to sum
    let total: i32 = nums4.iter().sum();
    println!("{:?}", total);

    // flat_map(adaptor ) flatten all elements of vec into single iterator
    let flattened_into_chars: Vec<char> = hw.iter().flat_map(|sli| sli.chars()).collect();
    println!("{:?}", flattened_into_chars);

    //enumerate(adaptor) to add an index to each element
    let idx_char: Vec<(usize, &char)> = abc.iter().enumerate().collect();
    println!("{:?}", idx_char);
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
fn concurrency_basics() {
    let (tx, rx) = mpsc::channel();

    // creates new thread
    // move creates ownership
    thread::spawn(move || {
        let vals = vec![
            String::from("joey"),
            String::from("from"),
            String::from("the"),
            String::from("city"),
            String::from("of"),
            String::from("light"),
        ];

        for val in vals {
            // send transfer
            tx.send(val).unwrap();
            // timeout se we can see it in human speed
            thread::sleep(Duration::from_secs(1));
        }
    });

    // received
    for received in rx {
        println!("Message from another thread {}", received)
    }
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

#[derive(Debug)]
enum CustomError {
    MyError(()),
}
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//                      THE '?' Operator
// 🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞
//                      With Custom Error Handling
// 🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

fn first(n: i32) -> Result<i32, CustomError> {
    match n {
        5 => Ok(n),
        _ => Err(CustomError::MyError(println!("\n{}\n", "not 5".red()))),
    }
}

fn second() -> Result<i32, CustomError> {
    let x = first(8)?;
    Ok(x)
}

// 🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞
//                      With Box<dyn Error> Handling
// 🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞🦐🦑🦀🦞
fn one() -> Result<i32, String> {
    Ok(7)
}

fn two() -> Result<i32, Box<dyn Error>> {
    let x = one()?;
    Ok(x)
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

fn or_else_with_results() {
    // this is just a Err
    let result: Result<i32, &str> = Err("Error");
    // we can use or to provide a default in the instance of Err
    let value: Result<i32, &str> = result.or(Ok(42));

    println!("Value: {:?}", value); // Output: Value: Ok(42)

    // another Err
    let result_2: Result<i32, &str> = Err("Error");
    // the same as `.or()` but with `.or_else()` has a closure
    let value_2: Result<i32, &str> = result_2.or_else(|_| Ok(69));

    println!("Value: {:?}", value_2); // Output: Value: Ok(69)
}

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

fn or_else_with_options() {
    // this is just a None
    let result: Option<u32> = None;
    // we can use or to provide a default in the instance of None
    let value = result.or(Some(42));

    println!("Value: {:?}", value); // Output: Value: Some(42)

    // another Err
    let result_2: Option<u32> = None;
    // the same as `.or()` but with `.or_else()` has a closure
    // notice using this with opt vs result there is no closure arg
    let value_2 = result_2.or_else(|| Some(69));

    println!("Value: {:?}", value_2); // Output: Value: Some(69)
}
