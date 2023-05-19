// Reference Counting, Rc<T>
//
// In some cases, a single value might have multiple owners.
// For instance, a node in a graph, that has multiple edges.
// We do not want to free up this node from memory, as long as there are edges connected to it.

// To enable multiple ownership of a value, we can use a Reference Counting smart pointer,
// which will keep track of the amount of references to a value, and when there are no more references,
// the value will get cleaned up.

// Analogy: The first person that enters the room turns on the TV. When more people enter the room, they start
//  watching the TV. When the last person leaves the room, the person turns off the TV.
//  If the TV would be turned off while people were still watching, there would be panic :)

// The reference counting smart pointer is used when we want to allocate a value on the heap and have multiple
// parts of our program, READ that value, and we do not which part of our program is going to finish using that value
// last, at compile time. Otherwise, we would simply make that part of the program the owner of the value.

use std::rc::Rc;

// NOTE: The reference counting pointer we use here, is only useful for single-threaded programs.
use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // ::clone only increments the reference count, so no "deep cloning" occurs here.
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    rc_example();
}

fn rc_example() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}









