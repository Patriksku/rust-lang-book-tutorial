/*
The orphan rule from chapter 10, states that we can implement a trait on a type, as long as the trait
or the type, is defined within our crate.

The newtype pattern allows us to get around this restriction.
We do this by creating a tuple struct with one field, which is going to be the type we are wrapping.
    - This thin wrapper around our type is local to our crate, so we can implement any trait we like for it.

In this example, we want to implement the Display trait for a vector, but the Display trait and the vector type,
are both defined outside of our crate. So to get around the orphan rule, we create a tuple struct, which
stores the vector, and then we can implement the Display trait for Wrapper.
    - In our implementation, we simply access the vector by calling .0 on self.

The downside of this patterns is that Wrapper is a new type, so we can not call methods defined on the vector type,
directly on Wrapper. However, if we did want our newtype to implement every method of the type it is holding,
then we can implement the deref trait, such that dereferencing Wrapper, would return the inner value!

However, if we only wanted our newtype to have a subset of methods defined on the inner type, then we would have to
implement each of those methods manually.
*/

use core::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn main() {
    let w = Wrapper(vec!["hello".to_string(), "world".to_string()]);
    println!("w = {}", w);
}
