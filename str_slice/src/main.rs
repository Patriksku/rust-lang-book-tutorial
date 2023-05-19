// https://users.rust-lang.org/t/why-am-i-able-to-mutate-a-string-literal/39778
// https://stackoverflow.com/a/75316881
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=afb79d7cc9b56e147b93fa84ae444197
fn main() {
    let mut s = String::from("hello world");
    let s2 = "hello world";

    let word = first_word(&s);
    // s.clear();
    println!("the first word is: {}", word);

    // array slice
    let mut a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
    // a[0] = 50;
    println!("{}", slice[0]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
