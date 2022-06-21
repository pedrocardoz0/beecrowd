use std::io;

fn main() {
    let mut _a = String::new();
    let mut _b = String::new();
    let mut _c = String::new();
    let mut _d = String::new();

    io::stdin().read_line(&mut _a).unwrap();
    io::stdin().read_line(&mut _b).unwrap();
    io::stdin().read_line(&mut _c).unwrap();
    io::stdin().read_line(&mut _d).unwrap();

    let a: i32 = _a.trim().parse().unwrap();
    let b: i32 = _b.trim().parse().unwrap();
    let c: i32 = _c.trim().parse().unwrap();
    let d: i32 = _d.trim().parse().unwrap();

    let prod_ab: i32 = a * b;
    let prod_cd: i32 = c * d;
    let dif: i32 = prod_ab - prod_cd;

    println!("DIFERENCA = {}", dif);
}
