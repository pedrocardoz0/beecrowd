use std::io;

fn main() {
    let mut input_values = String::new();

    io::stdin().read_line(&mut input_values).unwrap();
    let mut array_values: Vec<&str> = input_values.split(" ").collect();

    let a: i32 = array_values[0].trim().parse().unwrap();
    let b: i32 = array_values[1].trim().parse().unwrap();
    let c: i32 = array_values[2].trim().parse().unwrap();

    let maior_ab: i32 = (a + b + i32::abs(a - b)) / 2;

    if maior_ab > c {
        println!("{} eh o maior", maior_ab);
    } else {
        println!("{} eh o maior", c);
    }
}
