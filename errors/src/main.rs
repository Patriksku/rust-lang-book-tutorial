// Result enum for recoverable errors!
// Similar to Option enum (Some, or None), but has Ok, or Err instead.

// In general, we want to use Result enum for error handling and error propagation.
// For testing, or other cases such as experimenting, or PoC:ing something, we can use panic, unwrap and expect.
// We can also use expect or unwrap when we know for a certain that the call will succeed.

// A good practice is to use custom types for validation!

use std::{fs::File, io::ErrorKind};

use custom_type::Guess;
mod custom_type;
mod error_propagation;

fn a() {
    b();
}

fn b() {
    c(20);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Wtf!");
    }
}

fn main() {
    a();
    // recoverable_error_basic();
    // recoverable_error();

    // Custom type for validation.
    let guess = Guess::new(100);
    println!("value of Guess is: {}", guess.value())
}

fn recoverable_error_basic() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Above can be (basically) rewritten to:
    let f = File::open("hello.txt").unwrap();

    // If we want to be able to alter the error message, we can simply do:
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn recoverable_error() {
    let f = File::open("hello.txt");

    // UGLY way of handling errors... -->
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // A LOT nicer way of handling errors... -->
    // Here we use something called Closures (will be covered in future tutorial).
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
