// Works.

pub fn example1() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("abc");
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("longest text is: {}", result);
}

// Good to note, is that the lifetime of our return value, always has to be tied to the lifetime of one of our parameters, always.
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
