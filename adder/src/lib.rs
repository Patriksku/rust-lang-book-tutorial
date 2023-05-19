// The params passed into assert eq and assert ne, need to implement the PartialEq and Debug traits.
// ALL the primitive values, and most of the standard library, have these two traits implemented.

// If we create or own structs and enums, we will have to implement them ourselves.
// 'cargo test' compiles your code in test mode, and runs the resulting test binary.

// There are 2 sets of command line options. One set goes to the 'cargo test' command, and one goes
// to the resulting test binary. These sets are separated by '--'.
// cargo test --> cargo test --help
// resulting cargo test binary --> cargo test -- --help

// By default, standard output is captured for passing tests, and we do not see it on the screen.
// We can use cargo test -- --show-output to change this default behavior.

// To run a specific test, write 'cargo run `test_name`'
// We can also run a subset of test by specifying a common part in their names -->
// e.g. 'cargo run add' --> would run all tests that have 'add' in their names.

// You can also run tests based on the module --> cargo test lib2::
// You can run all ignored tests --> cargo test -- --ignored

// Rust community categorizes tests as Unit tests, and Integration tests.

// UNIT tests
// ...are small, focused, test one module in isolation, and could test private interfaces.
// --> Live in the same file as our product code.

// INTEGRATION tests
// ...are completely external to your library, and thus, test the public interface of your library.
// --> Live in a folder called 'tests' at the root of your project.
// --> Cargo will turn each file inside of the 'tests' folder into a Crate.

mod lib2;
pub mod lib3;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            );
        }

        Guess { value }
    }
}

// Product code is in Default module, and tests are in the Test module
#[cfg(test)]
mod tests {
    // We bring the product code into scope.
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        // assert_ne!(4, add_two(2));
    }

    // Custom error message on failure!
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result,
        );
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // Here, we expect a specific panic message. This way, we know that the program panicked
    // in the way that we were expecting. We also get the panic!() output error message if it panics in any other way.
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        // Guess::new(-2);
        Guess::new(105);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four???"))
        }
    }
}
