// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let largest = extract_largest(number_list);
//     println!("The largest number is {}", largest);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let largest = extract_largest(char_list);
//     println!("The largest char is {}", largest);
// }
//
// fn extract_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
//     let mut largest = list[0];
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
// impl Point<f32> {
//     fn y(&self) -> f32 {
//         self.y
//     }
// }
// fn main() {
//     let p1 = Point { x: 1, y: 2 };
//     let p2: Point<f32> = Point { x: 2.4, y: 3.3 };
//     p1.x();
//     p2.x();
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p2 {:?}", p2);
    println!("p2 {:?}", p1);
}
