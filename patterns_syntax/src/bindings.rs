/*
The '@' operator let's us create a variable that holds a value, at the same time
we are testing that value, to see whether it matches a pattern.
 */
pub fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    // If a need exists, we can use '@' as in the first arm, otherwise we can do
    // as in the other arms. Here, we simply save the value to a new variable 'id_variable',
    // if it is within the specified range.
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
