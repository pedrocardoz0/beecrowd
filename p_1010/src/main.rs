use std::io;

fn main() {
    let mut product_one = String::new();
    let mut product_two = String::new();

    io::stdin().read_line(&mut product_one).unwrap();
    let mut splited_product_one: Vec<&str> = product_one.split(" ").collect();

    io::stdin().read_line(&mut product_two).unwrap();
    let mut splited_product_two: Vec<&str> = product_two.split(" ").collect();

    let cod_p_1: i32 = splited_product_one[0].trim().parse().unwrap();
    let num_p_1: f64 = splited_product_one[1].trim().parse().unwrap();
    let val_p_1: f64 = splited_product_one[2].trim().parse().unwrap();

    let cod_p_2: i32 = splited_product_two[0].trim().parse().unwrap();
    let num_p_2: f64 = splited_product_two[1].trim().parse().unwrap();
    let val_p_2: f64 = splited_product_two[2].trim().parse().unwrap();

    let total: f64 = ((num_p_1 * val_p_1) + (num_p_2 * val_p_2));
    println!("VALOR A PAGAR: R$ {:.1$}", total, 2);
}
