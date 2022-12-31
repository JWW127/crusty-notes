#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_parens)]

//single line comment
/* multi line comment */
/// docs comment
// u8 ---> 8bits
// usize ---> 64bits
// char ---> 4 bytes, largest utf8 ---> 4 bytes
// {:#?} pretty format for collections

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
    // NOTICE len is number of bytes not number of chars/elements
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

