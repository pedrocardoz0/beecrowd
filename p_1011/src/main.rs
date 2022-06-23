use std::io;

fn main() {
    let mut raio = String::new();

    io::stdin().read_line(&mut raio).unwrap();

    let _raio: f64 = raio.trim().parse().unwrap();
    let _raio_pow_3: f64 = f64::powf(_raio, 3.0);

    let volume = ((4.0 / 3.0) * 3.14159) * _raio_pow_3;

    println!("VOLUME = {:.1$}", volume, 3);
}
