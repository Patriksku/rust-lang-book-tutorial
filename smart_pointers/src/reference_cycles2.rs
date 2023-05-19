/*
If a parent is dropped, all its children should be dropped too, but that is not the case the other way around.
That is why we use a Weak<> sp to the parent, but an Rc<> sp to the children.
    The parents own the children. The children do not own their parents.

Weak<> is a non-owning reference to something (Weak reference does not count towards ownership!).
Rc<> is an owning reference to something.



*/

use std::{
    borrow::Borrow,
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn main() {
    println!();
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // strong_count --> Number of references that have ownership of the data.
    // weak_count --> Number of references that do no have ownership of the data.
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // upgrade() because the underlying value might have been dropped.
    // returns an optional. Some() will be Rc<Node>.
    // We have to use upgrade() in these scenarios, because the Weak<> sp has no idea
    // if the underlying value is there.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // parent expects Weak<Node>. So in order to downgrade Rc<Node> to Weak<Node>,
    // we call downgrade() on a reference to Rc<Node>.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    visualize_strong_weak_count();
}

fn visualize_strong_weak_count() {
    println!();
    println!("----------------");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        // branch will still have weak_count 1 here, but weak references do not count towards ownership,
        // and will therefore be dropped here at the end of its scope.
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
