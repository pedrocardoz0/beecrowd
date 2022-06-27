use std::io;

fn main() {
    let mut _time = String::new();
    let mut _speed = String::new();

    io::stdin().read_line(&mut _time).unwrap();
    io::stdin().read_line(&mut _speed).unwrap();

    let time: i64 = _time.trim().parse().unwrap();
    let speed: i64 = _speed.trim().parse().unwrap();

    let distance: f64 = (time * speed) as f64;

    let liters = distance / 12.0;

    println!("{:.1$}", liters, 3);
}
