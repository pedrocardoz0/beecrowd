use std::io;

fn main() {
    let mut distance_km = String::new();
    let mut gas_liter = String::new();

    io::stdin().read_line(&mut distance_km).unwrap();
    io::stdin().read_line(&mut gas_liter).unwrap();

    let _distance_km: f64 = distance_km.trim().parse().unwrap();
    let _gas_liter: f64 = gas_liter.trim().parse().unwrap();

    let consume: f64 = _distance_km / _gas_liter;

    println!("{:.1$} km/l", consume, 3);
}
