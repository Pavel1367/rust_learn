fn main() {
    let x = 32;
    println!("The value is {}", x);
    let x = "SIX";
    println!("The value is {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("The value is {}", SUBSCRIBER_COUNT);
    my_function(2, 5)
}

fn my_function(x: i32, y: i32) {
    println!("This is a function");
    println!("The value is {}", x);
    println!("The value is {}", y);
}
