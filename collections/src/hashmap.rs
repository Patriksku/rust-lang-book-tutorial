use std::collections::HashMap;

pub fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // Without "&" (reference), we actually move the strings into the HashMap.
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Iterating over HashMap.
    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    // If "Yellow" does not exist, create the KV entry, otherwise, do nothing.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(30); // will do nothing.

    // Map each word into HashMap with value being that words occurrence.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
