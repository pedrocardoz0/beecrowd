use std::io;

fn main() {
    let mut input_values = String::new();

    io::stdin().read_line(&mut input_values).unwrap();
    let mut array_values: Vec<&str> = input_values.split(" ").collect();

    let a: f64 = array_values[0].trim().parse().unwrap();
    let b: f64 = array_values[1].trim().parse().unwrap();
    let c: f64 = array_values[2].trim().parse().unwrap();

    let triangulo: f64 = (a * c) * 0.5;
    let circulo: f64 = f64::powf(c, 2.0) * 3.14159;
    let trapezio: f64 = ((a + b) / 2.0) * c;
    let quadrado: f64 = b * b;
    let retangulo: f64 = a * b;

    println!("TRIANGULO: {:.1$}", triangulo, 3);
    println!("CIRCULO: {:.1$}", circulo, 3);
    println!("TRAPEZIO: {:.1$}", trapezio, 3);
    println!("QUADRADO: {:.1$}", quadrado, 3);
    println!("RETANGULO: {:.1$}", retangulo, 3);
}
