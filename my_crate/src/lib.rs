// Rust will run any tests we have in the doc comments of our different functions.
// Rust will also generate a .html documentation for us based on our docs.
// We can take a look at it like so: 'cargo doc --open'

// Different sections we can add to comments:
// https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html

// To document the item containing the documents, use '//!'
// Below comment is inside lib.rs, so it is documenting our library.

//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
