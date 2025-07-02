fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &a[..2];
    println!("The value is {:?}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
