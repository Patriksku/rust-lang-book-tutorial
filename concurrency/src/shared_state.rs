/*
Shared State
With Shared State concurrency, we have some piece of data in memory, that multiple threads can read and write to.

Mutex = Mutual Exclusion.
Means that you have one piece of data, and only one thread can access this piece of data at any given time.


The Rust programming language itself provides few concurrency features, but it does give you the building blocks
to create your own concurrency features, or use concurrency features written by others.
Two basic concurrency concepts provided as building blocks by the standard library, are the
Send and Sync traits.

Note that if you are doing simple numerical operations,
there are types simpler than Mutex<T> types provided by the std::sync::atomic module of the standard library.
These types provide safe, concurrent, atomic access to primitive types.
We chose to use Mutex<T> with a primitive type for this example so we could concentrate on how Mutex<T> works.
 */

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

pub fn main() {
    let m = Mutex::new(5);

    {
        // After calling lock() and unwrap(), we get a MutexGuard<> sp,
        // whose deref trait points to the inner data of the Mutex<>,
        // which in this case, is the Integer 5.
        //
        // MutexGuard<> also implements the drop traits, so that when MutexGuard<>
        // goes out of scope, then it releases the lock to the data. This means that
        // the lock gets released automatically, and you don't have to remember to do it.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
    atomic_example();
}

// For our example we want the functionality of Rc<>, because we can't move counter inside the for-loop,
// as it would be owned by the first iteration, and then not available for all other iterations.
// So if we could have multiple owners it would solve the problem. But Rc<> is not thread safe.
// The thread-safe alternative of Rc<> is Arc<>. (A stands for atomic).
//
// Atomics are like primitive types, except that they can be shared across threads (thread-safe).
//
// Counter is immutable, but we are able to get a mutable reference to the value that it holds.
// That is because a Mutex uses interior mutability.
//
// In the same way that the RefCell<> allows to mutate a value inside an Rc<>,
// The Mutex<> allows you to mutate a value inside an Arc<>.
//
// Also, the RefCell<> comes with the risk of creating circular dependencies,
// While the Mutex<> comes with the risk of creating deadlocks.
fn atomic_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // We shadow counter to solve the move issue mentioned above.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
