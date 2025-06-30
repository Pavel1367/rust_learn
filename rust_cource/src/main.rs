use std::collections::HashMap;

fn main() {
    let vec: Vec<[i32; 2]> = Vec::new();
    let mut scores = HashMap::new();
    scores.insert(String::from("Pavel"), 15);

    let score = scores.get("Pavedl").unwrap_or(&0);
    println!("Score: {}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
