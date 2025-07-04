use unicode_segmentation::UnicodeSegmentation;

// fn main() {
//     let hello = String::from("Привет");
//     for b in hello.bytes() {
//         println!("{}", b);
//     }
//     println!("{}", hello);
//
//     for c in hello.chars() {
//         println!("{}", c);
//     }
//
//     for g in hello.graphemes(true) {
//         println!("{}", g);
//     }
// }

use std::collections::HashMap;

fn main() {
    let text = "Hello wonderful world this wonderful world!";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
