use std::io;

fn main() {
    let mut _employeeNumber = String::new();
    let mut _employeeHours = String::new();
    let mut _employeeHourPrice = String::new();

    io::stdin().read_line(&mut _employeeNumber).unwrap();
    io::stdin().read_line(&mut _employeeHours).unwrap();
    io::stdin().read_line(&mut _employeeHourPrice).unwrap();

    let employeeNumber: i32 = _employeeNumber.trim().parse().unwrap();
    let employeeHours: i32 = _employeeHours.trim().parse().unwrap();
    let employeeHourPrice: f32 = _employeeHourPrice.trim().parse().unwrap();

    let salary: f32 = employeeHours as f32 * employeeHourPrice;

    println!("NUMBER = {}", employeeNumber);
    println!("SALARY = U$ {:.1$}", salary, 2);
}
