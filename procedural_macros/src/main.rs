/*
Procedural macros are like functions, they take code as input, operate on that code,
and produce code as output.

There are 3 kinds of procedural macros:
1. Custom derived,
2. Attribute-like,
3. Function-like.

They all work in a similar fashion. For complex technical reasons, which the Rust team hopes to fix,
procedural macros must be defined in their own crate, with a custom crate type.
The 3 kinds of procedural macros are all defined using a similar syntax:
    - The name of the function is the name of our procedural macro.
    - I/O is a TokenStream.
    - Attribute above fn() which specify the kind of procedural macro we are creating.

Tokens are the smallest individual elements of a program. They can represent keywords, identifiers, operators,
separators or literals.



First thing we need to do is create a new lib crate called hello_macro.
    cd ..
    cargo new hello_macro --lib



Attribute-like macros are similar to custom derived macros, except, instead of generating code for the derived
attribute, we can create a custom attribute. Also, custom-derived macros only work on structs and enums, 
while attribute-like macros work on other types such as functions.


Function-like macros look like function calls, however, they are more flexible.
    - They can take a variable number of args, 
    - They operate on Rust code.

 */
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

 #[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}


// ***Attribute-like macros***
// E.g. web server, where we create a new attribute 'route' that takes in the method and path.
// #[route(GET, "/")]
// fn index() {
//     //...
// }

// pub fn route(
//     attr: TokenStream, // GET, "/"
//     item: TokenStream // fn index() {}
// ) -> TokenStream {
//     //...
// }


// ***Function-like macros***
// let sql = sql!(SELECT * FROM posts WHERE id=!);

// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
//     //...
// }
