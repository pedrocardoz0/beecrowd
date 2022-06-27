use std::io;

fn main() {
    let mut _distance = String::new();

    io::stdin().read_line(&mut _distance).unwrap();

    let distance: f64 = _distance.trim().parse().unwrap();

    let minutes: f64 = distance * 2.0;

    println!("{:.1$} minutos", minutes, 0);
}
