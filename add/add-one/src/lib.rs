use rand;

pub fn addOne(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(3, addOne(2))
    }
}
