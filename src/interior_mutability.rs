use std::cell::Cell;
// interior mutability is the idea of being able to change something within an immutable container

pub struct Turtle {
    name: String,
    is_fed: Cell<bool>,
}

impl Turtle {
    fn new(name: String) -> Self {
        Turtle {
            name,
            is_fed: Cell::new(false),
        }
    }

    fn feed(&self) {
        self.is_fed.set(true);
    }

    fn is_hungry(&self) -> bool {
        self.is_fed.get()
    }
}

pub fn test_interior_mut() {
    let turtle = Turtle::new("Gary".to_string());
    println!("should I feed {}: {}", turtle.name, turtle.is_hungry());
    // should I feed Gary: False
    turtle.feed();
    println!("should I feed {}: {}", turtle.name, turtle.is_hungry());
    // should I feed Gary: True
}
