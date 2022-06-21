use std::io;

fn main() {
    let mut _a = String::new();
    let mut _b = String::new();

    io::stdin().read_line(&mut _a).unwrap();
    io::stdin().read_line(&mut _b).unwrap();

    let a: f32 = _a.trim().parse().unwrap();
    let b: f32 = _b.trim().parse().unwrap();

    let media: f32 = ((a * 3.5) + (b * 7.5)) / 11.0;

    println!("MEDIA = {:.1$}", media, 5);
}
