// traits define behaviour that can be shared by types
// traits move errors related to undefined behaviour from runtime to compile time

// traits also need to be brought into scope when importing externally
// external traits can't be implemented on external types (coherence)
use std::fmt::{Debug,Display};

pub trait Summary {
    // if we didn't add the body for this function we would need to add it in all trait
    // implementations
    // it isn't possible to call the default implementatin from an overriding implementation of
    // the same method
    fn summarize_author(&self) -> String {
        String::from("Unknown")
    }

    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// building an enum of structs is a bit clunky
enum Content {
    NewsArticle(NewsArticle),
    Tweet(Tweet),
}


//if you have multiple parameters and want them to have the same type you would need to use trait
//bounds: pub fn notify<T: Summary + Debug>(item: &T) {
fn notify(item: &(impl Summary + Debug)) {
    println!("{}", item.summarize());
}

fn get_a_tweet() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}


fn main() {
    let t = Content::Tweet(Tweet {
        username: String::from("adrop"),
        content: String::from("hello"),
        reply: false,
        retweet: false,
    });

    let a = Content::NewsArticle(NewsArticle {
        headline: String::from("hello"),
        location: String::from("here"),
        author: String::from("me"),
        content: String::from("world"),
    });

    match t {
        Content::NewsArticle(i) => println!("{}", i.summarize()),
        Content::Tweet(i) => println!("{}", i.summarize()),
    }

    if let Content::NewsArticle(arr) = a {
        notify(&arr);
    }

    println!("{}", get_a_tweet().summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// cmp_display works only if T implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
