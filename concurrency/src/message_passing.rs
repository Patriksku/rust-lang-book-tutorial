use std::{sync::mpsc, thread, time::Duration};

// Message passing is one approach of safe concurrency.
// You have threads that pass messages among each other.
//
// Rust slogan:
// "Do not communicate by sharing memory, instead - share memory by communicating!"
//
// Channels
// Channels are analogous to a channel of water, like a river or a stream.
// If you put a rubber duck or boat on the stream, it will travel downstream.
// A channel in programming has two halfs:
//  - A transmitter,
//  - A receiver.
//
// The transmitter is the upstream. The receiver is the downstream (where the duck will end up).
// - A channel is said to be closed, if either the transmitter or receiver half is dropped.
//
// Channels can be used in e.g. a chat system, or a program with many threads,
// where many threads perform a part of a calculation, and one thread aggregates the results.
//
// - You can have many producers of messages, but only one receiver.
//
pub fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rx.recv(), or rx.try_recv().
    // rx.try_recv() also exists, which will not block the main thread.
    // this means that it will instead return a Result<> immediately,
    // which will be OK if a value is available, else error if no values are available at the time.
    //
    // very useful if we want the thread to do other work. E.g, we can have a loop where we sometimes check for messages,
    // at other times, we do other work.

    // rx treated as an iterator here.
    // every iteration will have a value that we pass down the channel.
    // when the channel closes, the iteration will end.
    for received in rx {
        println!("Got: {}", received);
    }
}
