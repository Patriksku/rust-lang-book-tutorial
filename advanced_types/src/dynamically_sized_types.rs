/*
Dynamically sized types (DSTs, unsigned types) are types whose size
we can only know at runtime.

In general, DSTs are used like bellow, we use '&' version of the types,
that has an extra bit of metadata that stores the size of the dynamic information.

The golden rule for DSTs is that we must always put the behind some sort of pointer.
bellow 'str' could then be put as a '&str', or e.g. behind a Box<>, or Rc<>.


Traits are also DSTs. E.g. Trait Objects, that have to be behind some sort of pointer.
Rust has a special Trait called 'Sized', used for determining if a type's size can be
known at compile time or not.
- The Sized trait is automatically implemented for every type whose size is known at compile time.
- Also, Rust implicitly adds the Sized trait bound to every generic fn():

    fn generic<T>(t: T) {}
    
    -> Rust will convert to:

    fn generic<T: Sized>(t: T) {}

So by default, generic fn() will only work with types whose size is known at compile time.
However, you can use the special syntax bellow to relax this restriction:

    fn generic<T: ?Sized>(t: &T) {}

This means that T may be Sized, or not. This special syntax is only available for the Sized trait.
Because T might not be Sized, we have to put it behind some sort of pointer, e.g. '&T'.


 */

// We do not know the size of 'str' at compile time. Instead, we can use &str.
// An '&str' will store 2 values:
// -> 1. Address pointing to the location of the str in memory.
// -> 2. Length of the str.
//
// Both have a type of 'usize', so we know the size of '&str' at compile time.
pub fn main() {
    // let s1: str = "Hello";
    // let s2: str = "Ok.";

    let s1: &str = "Hello";
    let s2: &str = "Ok.";
}