/*
You might have a trait that is dependent on functionality from another trait.
In this case, your trait is dependent on the other trait being implemented.

The trait you rely on is called the supertrait.
 */

use std::fmt;

// Here, we get an error on to_string(), because we do not know if self implements to_string().
// to_string() is defined in the Display trait, so we want to make sure that anything that implements
// OutlinePrint, also implements the Display trait. To encode this relationship, we simply add ': fmt::Display'
// after our trait name, like below.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
pub fn main() {}
