use std::io;

fn main() {
    let mut _a = String::new();
    let mut _b = String::new();

    io::stdin().read_line(&mut _a).unwrap();
    io::stdin().read_line(&mut _b).unwrap();

    let a: i32 = _a.trim().parse().unwrap();
    let b: i32 = _b.trim().parse().unwrap();

    let prod: i32 = a * b;
    println!("PROD = {}", prod);
}
