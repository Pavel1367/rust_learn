fn main() {
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("Three")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
