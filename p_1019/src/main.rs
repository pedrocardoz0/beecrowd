use std::io;

fn main() {
    let mut seconds = String::from("");

    io::stdin().read_line(&mut seconds).unwrap();

    let mut seconds = seconds.trim().parse::<u32>().unwrap();

    let hours = seconds / 3600;
    seconds = seconds % 3600;

    let minutes = seconds / 60;
    seconds = seconds % 60;

    println!("{}:{}:{}", hours, minutes, seconds);
}
