// We have to add our 'adder' library into scope,
// because every file in the 'tests' folder (like this one),
// is going to be a new Crate.

// One thing to note -> we have lib.rs in src/. This means we have a library crate.
// If we had a main.rs file, we would have a binary crate.
// We CAN'T directly tests binary crates with integration tests.
// This is why it is common to see a binary crate, that is a thin wrapper around a library crate.
// This way, you can test a library crate with integration tests.

// If we want to only run our integration tests, we would type:
// 'cargo test --test integration_test'

// We have all common code in subfolders inside of 'tests' folder, because all files
// in root of 'tests' folder are their own crates. Files in subfolders do not get compiled as crates.
use adder::*;

// mod.rs in subfolder --> special name of file which make all code inside of mod.rs usable for other integration test files.
// This is a module declaration. It will look for the contents of the module, either in a file called
// 'common.rs', or a folder called 'common' with a file called mod.rs.
mod common;

// We do not need a tests module annotated with '#[cfg(test)]', for files in 'tests' folder.
// Also, we can only call the public API, not
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::lib3::add_two(2))
}
