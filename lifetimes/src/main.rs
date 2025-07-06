// fn main() {
//     let str1 = String::from("Helloh");
//     {
//         let str2 = String::from("World");
//         let result = longest(str1.as_str(), str2.as_str());
//         println!("{}", result);
//     }
// }
//
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn main() {}
