use std::io;

fn main() {
    let mut name = String::new();
    let mut _salary = String::new();
    let mut _seels = String::new();

    io::stdin().read_line(&mut name).unwrap();
    io::stdin().read_line(&mut _salary).unwrap();
    io::stdin().read_line(&mut _seels).unwrap();

    let salary: f64 = _salary.trim().parse().unwrap();
    let seels: f64 = _seels.trim().parse().unwrap();

    let comission: f64 = seels * 0.15;
    let total: f64 = salary + comission;

    println!("TOTAL = R$ {:.1$}", total, 2);
}
