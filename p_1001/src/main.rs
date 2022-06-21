use std::io::{self, Read};
fn main() {
    let mut _a: String = String::new();
    let mut _b: String = String::new();
    let mut x: i32 = 0;

    io::stdin().read_line(&mut _a);
    io::stdin().read_line(&mut _b);

    let a: i32 = _a.trim().parse().expect("error");
    let b: i32 = _b.trim().parse().expect("error");

    x = a + b;

    println!("X = {}", x);
}
