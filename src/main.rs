#![allow(dead_code)]
#![allow(unused_parens, unused_variables)]

//single line comment
/* multi line comment */
/// docs comment
// u8 ---> 8bits
// usize ---> 64bits
// char ---> 4 bytes, largest utf8 ---> 4 bytes
// {:#?} pretty format for collections
// '_' == All/NA
// '_name' == silence warnings will use later
use std::fmt::Display;
use std::str::FromStr;
use strum_macros::EnumString;

fn main() {}

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
    // NOTICE len is number of bytes for a char not number of elements
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

    // notice capacity jumps to 8 when we have 5 items
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
    //you can asign conditionals similar to funcs in js
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
    }

    let my_rect = Rect {
        height: 5,
        width: 5,
    };

    println!("{:?}", my_rect.get_area()); //25
    println!("{:?}", my_rect.get_perimeter()); //20
    dbg!(my_rect);
    //dbg is for quick debugging basically pretty prints. be aware it also takes ownership
    //moving dbg! above our prints will cause a mover error
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

//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
//▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
