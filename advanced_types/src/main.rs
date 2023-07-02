/*
Newtype pattern - Continuation

We can also use the newtype pattern to increase type-safety.
One example is if we have two functions, that both take an u32 arg,
one fn takes 'id', and one fn takes 'age'.

Instead of passing in a raw u32 parameter, we could use a tuple struct
to ensure that only that particular argument can be passed:
    struct Age(u32);
    struct Id(u32);

Another use for the newtype patterns is to abstract away implementation details.
E.g. We can create a people type that wraps a HashMap of integers to strings.
Code using the people type will only be exposed to the public API, and will have
no knowledge of the datatype being used internally.
-> In general, the newtype pattern is a lightweight way to achieve encapsulation.


Type Aliases
In addition to the newtype pattern, Rust allows you to create type Aliases to give
existing types new names. So "Kilometers" bellow is basically a synonym for the i32 type.

The main use of Type Aliases is to reduce repetition. We can also use Type Aliases to 
convey meaning, e.g. "Thunk" is a word for code that will be evaluated at some later 
point in time. So this is an appropriate name for a Clojure that gets stored.
 */
mod never_type;
mod dynamically_sized_types;

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    use_type_aliases_to_reduce_repetition();

    never_type::main();
    dynamically_sized_types::main();
}

// We can swap out the long var/arg definitions against a Type Alias, that contains
// the same type, like bellow.
fn use_type_aliases_to_reduce_repetition() {
    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    type Thunk = Box<dyn Fn() + Send + 'static>;

    // fn aliases_ex1(f: Box<dyn Fn() + Send + 'static>) {}
    
    // fn aliases_ex2() -> Box<dyn Fn() + Send + 'static> {
    //     return Box::new(|| println!("hi"));
    // }

    fn aliases_ex1(f: Thunk) {}
    
    fn aliases_ex2() -> Thunk {
        return Box::new(|| println!("hi"));
    }
}


