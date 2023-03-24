fn simulated_expensive_fn() -> String {
    "expensive".to_string()
}
// no closure 1
fn bad_strategy(input: u32) {
    // not very DRY code, notice we make a call for almost every logical path
    if input < 5 {
        println!(
            "strategy being built from input {}",
            simulated_expensive_fn()
        );
    } else if input > 500 {
        println!(
            "strategy being built from input {}",
            simulated_expensive_fn()
        );
    } else if input == 42 {
        println!("strategy being built from input 42",);
    } else {
        println!(
            "strategy being built from input {}",
            simulated_expensive_fn()
        );
    }
}

// no closure 2
fn slightly_better_strategy(input: u32) {
    // we can store the value of expensive_fn with a single call
    let store_expensive_result = simulated_expensive_fn();

    // but the downside is we are still calling the fn even when we dont need to
    if input < 5 {
        println!("strategy being built from input {}", store_expensive_result);
    } else if input > 500 {
        println!("strategy being built from input {}", store_expensive_result);
    } else if input == 42 {
        // hmmmmm???
        println!("strategy being built from input 42",);
    } else {
        println!("strategy being built from input {}", store_expensive_result);
    }
}

// Closure
fn closure_strategy(input: u32) {
    // closures store the definition not the result so its not called here
    let store_expensive_closure_definition = || "expensive".to_string();

    // only within our logical branch can it be called
    if input < 5 {
        println!(
            "strategy being built from input {}",
            store_expensive_closure_definition()
        );
    } else if input > 500 {
        println!(
            "strategy being built from input {}",
            store_expensive_closure_definition()
        );
    } else if input == 42 {
        // this doesnt matter since its not called until reached
        println!("strategy being built from input 42",);
    } else {
        println!(
            "strategy being built from input {}",
            store_expensive_closure_definition()
        );
    }
}
