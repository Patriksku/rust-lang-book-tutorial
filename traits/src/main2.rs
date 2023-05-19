// Trait bounds.

// below example from: https://doc.rust-lang.org/rust-by-example/generics/bounds.html

// When working with generics, the type parameters often must use traits as "bounds" to
// stipulate what functionality a type implements. For example, the following example
// uses the trait Display to print and so it requires T to be bound by Display;
// that is, T must implement Display.

// fn printer<T: Display>(t: T) {
//     println!("{}", t);
// }

use std::fmt::{Debug, Display};

use crate::Summary;

pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    // ...
}

// This function is different from above. Here, we specifically say that item1 and item2 have to be of the same type.
// This is called the trait bounds syntax.
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
//     // ...
// }

// Examples of implementing MORE than one trait:
// pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {
//     // ...
// }

// pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
//     // ...
// }

// How to use the "Where"-clause to make generics with implementations more readable:
// Example WITHOUT:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    //...
    1
}

// Example WITH:
fn some_function_with<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    //...
    1
}
