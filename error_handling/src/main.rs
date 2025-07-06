// fn main() {
//     a()
// }
// fn a() {
//     b()
// }
// fn b() {
//     c(228)
// }
// fn c(numb: i32) {
//     if numb == 22 {
//         panic!("Dont pass 22!")
//     }
// }
use std::fs::File;
use std::io::{ErrorKind, Read};
fn main() {
    // let f = File::open("Hello.txt");
    // let f = f.unwrap_or_else(|error| match error.kind() {
    //     ErrorKind::NotFound => match File::create("Hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => panic!("Problem creating the file: {:?}", e),
    //     },
    //     other_error => {
    //         panic!("Problem opening the file: {:?}", other_error)
    //     }
    // });
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    Ok(s)
}
