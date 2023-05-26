/*
Associated types are placeholders that you can add to your traits, which methods then can use.

Associated types vs. Generics:
    - Both allow us to define a type without specifying the concrete value.
    - The difference with associated types, is that you can only have one concrete type per implementation,
        whereas with generics, you can have multiple types per implementation.

When deciding between associated types and generics, the question is:
    - Does it make sense having multiple implementations for a single type, or just one implementation for that type?

In the case of the Iterator trait, it makes sense to use an associated type, because for any given implementation,
we want the next method to return the same concrete type.
 */

mod calling_methods_with_the_same_name;
mod default_generic_type_parameters;
mod generic_trait;
mod newtype_pattern;
mod supertraits;

// 'type Item;' is an associated type below.
// Item will be the thing we return. Then when we implement our Iterator trait, we will specify
// a concrete type for Item.
//
// This way, you can define a trait, which uses some type that is unknown, until we implement the trait.
// E.g:
//  - If the Iterator trait is implemented on a vector of integers, then Item will be an integer.
//  - If the Iterator trait is implemented on a string, then Item will be a char.
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    generic_trait::main();
    default_generic_type_parameters::main();
    calling_methods_with_the_same_name::main();
    supertraits::main();
    newtype_pattern::main();
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

// You can not have a different type here as mentioned above.
// Check 'generic_trait.rs' for generic implementation.
// -->
// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0)
//     }
// }
