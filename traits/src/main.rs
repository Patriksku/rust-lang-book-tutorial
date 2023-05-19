// Traits allow us to define a set of methods that are shared across different types.

use std::{fmt::format, iter::Sum};
mod conditional_implement_methods;
mod main2;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// All types implementing this trait need to specify all non-default implementations.
pub trait Summary {
    // We define the function name, param and return value, the rest is defined by the different types that implement this trait.
    fn summarize(&self) -> String;

    // We define a default function. This gives the types implementing this trait an option to either use this one,
    // or override it with own specific logic.

    // fn summarize(&self) -> String {
    //     String::from("(Read more...)")
    // }

    // Default implementations can call other methods inside the trait definition.
    // This
    fn summarize_author(&self) -> String;

    fn summarize_call(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Important restriction:
// We can only return one specific type implementing Summary.
// So returning either Tweet or NewsArticle based on a condition, would not be allowed.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Here, 'item' can be any type that implements Summary!
// Now this first function below is a simplification of the second function bellow.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize_call())
}

// In the background, the above 'notify' function translates to:
// This is called a trait bound:
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize_call())
// }

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The sky is Falling!"),
        content: String::from("The sky is actually not falling."),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize_call());
    notify(&article);

    // Method chaining:
    // Function returning implementation of trait.
    println!("{}", returns_summarizable().summarize());

    // check main2.rs.
}
