/*
We need to create a new lib crate inside this crate.
    cargo new hello_macro_derive --lib

Naming convention
If you have a custom derived macro, then you will name your crate:
    {current_crate_name}_derive


Each crate still has to be published seperately, and code using our crates,
has to bring each crate into scope. Which is why we saw 2 'use' statements
in 'procedural_macros'.

 */
pub trait HelloMacro {
    fn hello_macro();
}