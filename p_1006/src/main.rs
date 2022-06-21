use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();
    io::stdin().read_line(&mut c).unwrap();

    let a: f32 = a.trim().parse().unwrap();
    let b: f32 = b.trim().parse().unwrap();
    let c: f32 = c.trim().parse().unwrap();

    let media: f32 = ((a * 2.0) + (b * 3.0) + (c * 5.0)) / 10.0;

    println!("MEDIA = {:.1$}", media, 1);
}
