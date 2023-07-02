use declarative_macros::vec;

/*
Declarative Macros allows you to write code, that writes other code, a.k.a.
meta-programming. E.g. of a macro is println!(). This expands to produce more code
than you have written manually.

Writing macros allows you to reduce the amount of code you have to write and maintain,
which is similar to functions, however there are some key differences.
    - Functions must declare the amount of args they are expecting,
        while macros can accept a variable amount of args.
    - Functions are called at runtime, while macros are expanded before the program
        finishes compiling.
    - Macros are a lot more powerful to functions. The downside is that you are introducing
        more complexity, as macros are harder to read, understand and manage.

Rust has 2 forms of macros:
    - Declarative,
    - Precedural.


Declarative macros are the most widely used macros in Rust. They replace code based on patterns.


 */
fn main() {
    // As we can observe, the 'vec!' macro takes variable amount of args,
    // but can also take different types of args.
    let v1: Vec<u32> = vec![1, 2, 3];
    let v2: Vec<&str> = vec!["a", "b", "c", "d", "e"];
 }
