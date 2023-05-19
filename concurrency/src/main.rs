use std::{thread, time::Duration};

/*
Two main types of threads:
    1. 1-to-1 Threads (a.k.a. OS threads, native threads, system threads, etc.)
    2. Green threads (a.k.a. user threads, program threads, n-to-n threads, etc.)

Many OS provide an API to create new threads.
- 1-to-1 threads will map to an OS thread.
- Green threads are an implementation of threads within programming languages.
    - Do not have 1-to-1 mapping to OS threads, so you can e.g. have 20 green threads, mapped to only 5 OS threads.

- To keep the runtime minimal, Rust does not have inherent support for n-n threads. But you can of course use crates
    that provide this functionality.

 */

mod message_passing;
mod shared_state;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread would wait here for the spawned thread until it was finished, and then continue executing.
    // handle.join().unwrap();

    for i in 1..=5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    clojure();
    println!();

    // message_passing::main();
    shared_state::main();
}

fn clojure() {
    let v = vec![1, 2, 3];

    // Bellow would be illegal without move, because Rust does not know if the clojure will outlive the current function,
    // which is the owner of 'v'.
    // As 'v' is only used in a println() statement, Rust infers that it only needs a reference to 'v'.
    // By using move keyword, we force the clojure to take ownership of v -> problem solved.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
}
