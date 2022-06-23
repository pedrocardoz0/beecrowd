use std::io;

fn main() {
    let mut _x = String::new();
    let mut _y = String::new();

    io::stdin().read_line(&mut _x).unwrap();
    io::stdin().read_line(&mut _y).unwrap();
    let mut array_one: Vec<&str> = _x.split(" ").collect();
    let mut array_two: Vec<&str> = _y.split(" ").collect();

    let x1: f64 = array_one[0].trim().parse().unwrap();
    let y1: f64 = array_one[1].trim().parse().unwrap();
    let x2: f64 = array_two[0].trim().parse().unwrap();
    let y2: f64 = array_two[1].trim().parse().unwrap();

    let _wrapper_x: f64 = x2 - x1;
    let _wrapper_y: f64 = y2 - y1;

    let diff_x: f64 = f64::powf(_wrapper_x, 2.0);
    let diff_y: f64 = f64::powf(_wrapper_y, 2.0);
    let sum_xy = diff_x + diff_y;

    let distance: f64 = f64::powf(sum_xy, (1.0 / 2.0));
    println!("{:.1$}", distance, 4);
}
