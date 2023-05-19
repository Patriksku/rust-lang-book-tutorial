// Good information on String, &str, and string literals:
// -> https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str

// Just like a vector, a String can grow or shrink in size.
use std::fmt::format;

pub fn main() {
    // Strings are stored as a collection of UTF-8 encoded bytes.
    let s1 = String::new();
    let s2 = "inital contents";
    let s3 = s2.to_string();
    let s4 = String::from("ok");

    // Appending to Strings
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    // Appending to Strings, p2.
    // -> Here, we move ownership from st1 into st3, and then we are taking all chars in st2,
    // -> and appending them to the end of st3.
    let st1 = String::from("Hello, ");
    let st2 = String::from("world!");
    let st3 = st1 + &st2;

    // Appending to Strings, p3.
    // -> Here, the format macro does not take ownership of s1 or s2, so we can still use them after this call.
    let st3p3 = format!("{}{}", s1, s2);

    // Indexing
    // In UTF-8, strings can be from 1 to 4 bytes long, so one "character" is not necessarily always 1 byte long.
    let hello = String::from("नमस्ते");

    // Above represented in:...
    // Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    // Unicode scalar
    // ['न', 'म', 'स', '्', 'त', 'े']

    // Grapheme clusters
    // ["न", "म", "स्", "ते"]

    // We can't index into a string as below, because Rust does now know what representation we want.
    // We can iterate over these in all three representative states (but not Grapheme out-of-the-box, for this we need to import a crate).
    // let c = hello[0];
}
