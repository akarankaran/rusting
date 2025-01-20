use std::fmt::Display;

trait Summary {
    type ItemType;
    fn summarize(&self) -> Self::ItemType;
}

struct NewsArticle {
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    type ItemType = String;
    fn summarize(&self) -> Self::ItemType {
        format!("{}: {}", self.headline, self.content)
    }
}

struct Number {
    value: i32,
}

impl Summary for Number {
    type ItemType = String;
    fn summarize(&self) -> Self::ItemType {
        format!("Number is: {}", self.value)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 2023 is here!"),
        content: String::from("New features are introduced."),
    };

    let number = Number { value: 42 };

    println!("{}", article.summarize());
    println!("{}", number.summarize());
}