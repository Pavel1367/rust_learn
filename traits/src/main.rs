use std::fmt::{Display, format};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        '@'.to_string() + &self.username
    }
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Zhora"),
        content: String::from("ofc"),
        reply: false,
        retweet: false,
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    // let tweet = Tweet {
    //     username: String::from("Zhenia"),
    //     content: String::from("of course, as you probably already know, people"),
    //     retweet: false,
    //     reply: false,
    // };
    // let article = NewsArticle {
    //     author: String::from("Pavel"),
    //     headline: String::from("Purpose of books"),
    //     content: String::from("smth interesting here"),
    // };
    // println!("Tweet summary: {}", tweet.summarize());
    // println!("Article summary: {}", article.summarize());
    // notify(article);
    println!("Hello, world!{}", returns_summarizable().summarize_author());
}
