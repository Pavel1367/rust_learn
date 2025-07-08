struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn prints_returns_10(num: i32) -> i32 {
    println!("{}", num);
    10
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_returns_10(4);
        assert_eq!(10, value);
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 12,
            height: 10,
        };
        let smaller = Rectangle {
            width: 10,
            height: 15,
        };
        assert!(!larger.can_hold(&smaller));
    }
}
