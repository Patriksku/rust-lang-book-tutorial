// Box
// allows you to allocate values on the heap.
// Typically not used by themselves.
//
// Boxes don't have any overhead, except for storing the data on the heap,
// but they also don't have many other capabilites, so you would typically use them,
// in the following situations:
//
// -> When you have a type whose exact size can't be known at compile time,
//      and you want to use a value of that type, in a context which requires knowing the exact size.
//
// -> When you have a large amount of data, and you want to transfer ownership of the data, without copying
//      (because it is a large amount of data).
//
// -> When you own a value, and you only care that the value implements a specific trait, rather than
//      it being a specific type (a.k.a. Trait Object, covered later in chapter 17).
//
//
//
// Rust computes the size of non-recursive enums by going through each variant, and seeing how much size each 
// variant needs. For e.g. below.
//
//  enum Message {
//      Quit,                           // No space
//      Move { x: i32, y: i32 },        // 2 32-bit integers, etc...
//      Write(String),
//      ChangeColor(i32, i32, i32),
//  }
//
// Here, Rust will look at the variant taking up the most amount of space,
// because we can only use one variant of the enum at one time. 
// So the most space Message can take up, is equal to the most space the largest variant can take up!

// The Box smart pointer is going to be fixed size.
// It will point to an arbitrary amount of data on the heap, but on the stack,
// it is a fixed size pointer.

// So now we know exactly how much size we need for the Cons variant below.
// Instead of "?" amount of size needed, we now have "x" amount of size needed.

// Recursive
enum List {
    Cons(i32, Box<List>),
    Nil,
}

mod deref;
mod drop;
mod ref_counting;
mod interior_mutability;
mod ref_counting_mutable;
mod reference_cycles;
mod reference_cycles2;
use List::{Cons, Nil};

fn main() {
    // We are storing 5 on the heap, and in the stack, we are storing the pointer/memory address,
    // to the location.
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    deref::main();
    drop::main();
    ref_counting::main();
    interior_mutability::main();
    ref_counting_mutable::main();
    reference_cycles::main();
    reference_cycles2::main();
}

// Helper function to make code more readable, if want.
fn cons(i: i32, list: List) -> List {
    List::Cons(i, Box::new(list))
}
