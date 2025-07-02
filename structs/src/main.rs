#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn modify_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let mut rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect3 = Rectangle {
        width: 70,
        height: 10,
    };
    println!("rect: {:#?}", rect);
    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    rect2.modify_width(90);
    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    println!("The area of rectangle is {} square pixels.", rect.area())
}
