use std::io;

fn main() {
    let mut age = String::new();

    io::stdin().read_line(&mut age).unwrap();

    let _age: i64 = age.trim().parse().unwrap();

    let year = _age / 365;
    let remainder = _age % 365;

    let months = remainder / 30;

    let remainder = remainder % 30;

    println!("{} ano(s)", year);
    println!("{} mes(es)", months);
    println!("{} dia(s)", remainder);
}
