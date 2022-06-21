use std::io;

fn main() {
    let n: f64 = 3.14159;

    let mut _raio = String::new();
    let mut a: f64 = 0.0;

    io::stdin().read_line(&mut _raio);

    let raio: f64 = _raio.trim().parse().expect("error");
    a = n * f64::powf(raio, 2.0);

    println!("A={:.1$}", a, 4);
}
