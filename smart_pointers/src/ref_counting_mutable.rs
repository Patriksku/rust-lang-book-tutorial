use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn main() {
    // We wrap 5 in a RefCell sp, and then we wrap this in an Rc sp.
    // This allows us to mutably or immutably borrow 5, while allowing for multiple owners.
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Deref coercion here, we deref Rc into RefCell, and then get a mutable reference to the underlying value, and then
    // we dereference the mutable reference ('*' - follow the pointer), and change the value.
    *value.borrow_mut() += 10;

    println!();
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    println!("count of 'value' RC: {}", Rc::strong_count(&value));
    println!("count of 'a' RC: {}", Rc::strong_count(&a));
}