use std::{
    error::Error,
    fs::{self, File},
    io::{self, Read},
};

// Check the other method for simplified version!
pub fn read_username_from_file() -> Result<String, io::Error> {
    // By adding '?' at the end, we say that either f will get the file, or we will do an early return with the error that occurred.
    // SIMPLIFICATION:
    let mut f = File::open("hello.txt")?;

    // let f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // SIMPLIFICATION:
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // match f.read_to_string(&mut s) {
    //     Ok(file) => Ok(s),
    //     Err(e) => Err(e),
    // }
}

pub fn read_username_from_file_simplification() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_username_from_file_simplest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// main() is special, we cannot "return back" Error from it.
// this is one way of handling errors inside of main() instead of explicitly handling it.
// more about such things in future tutorials...
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
