use std::io;

fn main() {
    let mut _cash = String::new();

    io::stdin().read_line(&mut _cash).unwrap();

    let mut cash: i64 = _cash.trim().parse().unwrap();
    let cash_initial = cash;

    let cash_100: i64 = cash / 100;
    cash -= cash_100 * 100;

    let cash_50: i64 = cash / 50;
    cash -= cash_50 * 50;

    let cash_20: i64 = cash / 20;
    cash -= cash_20 * 20;

    let cash_10: i64 = cash / 10;
    cash -= cash_10 * 10;

    let cash_5: i64 = cash / 5;
    cash -= cash_5 * 5;

    let cash_2: i64 = cash / 2;
    cash -= cash_2 * 2;

    let cash_1: i64 = cash;

    println!("{}", cash_initial);
    println!("{} nota(s) de R$ 100,00", cash_100);
    println!("{} nota(s) de R$ 50,00", cash_50);
    println!("{} nota(s) de R$ 20,00", cash_20);
    println!("{} nota(s) de R$ 10,00", cash_10);
    println!("{} nota(s) de R$ 5,00", cash_5);
    println!("{} nota(s) de R$ 2,00", cash_2);
    println!("{} nota(s) de R$ 1,00", cash_1);
}
