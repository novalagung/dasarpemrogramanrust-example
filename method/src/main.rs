mod models;

fn main() {
    let mut car = models::Car::new(
        String::from("Mercedes-Benz"),
        String::from("Vision Gran Turismo")
    );
    // println!("car: {:?}", car);

    let info = car.info();
    println!("info: {:?}", info);

    // car.congratulate(String::from("Sylvanas Windrunner"));

    car.set_manufacture_year(2013);
    let detailed_info = car.info();
    println!("detailed info: {:?}", detailed_info);
}
