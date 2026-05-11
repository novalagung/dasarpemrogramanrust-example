// A.25.2, A.25.3, A.25.4 - main
mod models;

fn main() {
    let mut car = models::Car::new(
        String::from("Mercedes-Benz"),
        String::from("Vision Gran Turismo"),
    );

    // A.25.2 - method info
    let info = car.info();
    println!("info: {:?}", info);

    // A.25.3 - method congratulate
    car.congratulate(String::from("Sylvanas Windrunner"));

    // A.25.4 - method set_manufacture_year
    car.set_manufacture_year(2013);
    let detailed_info = car.info();
    println!("detailed info: {:?}", detailed_info);
}
