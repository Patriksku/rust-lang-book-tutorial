use std::{cell::{RefCell, Ref}, rc::Rc, borrow::BorrowMut};
use List::*;

/*
Rust is known for being a memory-safe language, so it provides certain guarantees, like you cannot have data races.
However, it does not provide the same guarantee for memory leaks!

Rust makes it difficult, but not impossible, to create memory that is never cleaned up.
We can create memory leaks by using the Rc<>, RefCell<>...
-> We can e.g. make items reference each other in a cycle, which will create a memory leak.

This creates a memory leak, because at the end of main(), first 'b' (the pointer) gets cleaned up from the stack.
However, the heap does not get cleaned up (what the pointer pointed to). Because it holds a reference to 'a',
which holds a reference to 'b' etc.... So both a and b will get cleaned up from the stack, but not from the heap.
    So a and b are orphaned on the heap, nothing points to them, and therefore they won't get cleaned up.
 */
#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn main() {
    println!();
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        // if let Some(link2) = link.borrow_mut().tail() {
        //     println!("wow is something....");
        // } else {
        //     println!("Is NIL");
        // }
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}