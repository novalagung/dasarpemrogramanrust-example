struct User {
    name: String,
    sign_in_count: u64,
    affliation: Vec<String>,
    active: bool,
}

struct Car {
    brand: String,
    model: String,
}

fn main() {
    let user_one = User{
        name: String::from("Orgrim Doomhammer"),
        sign_in_count: 12,
        affliation: vec![
            String::from("Warchief of the Horde"),
            String::from("Blackrock Chieftain"),
            String::from("The Doomhammer"),
        ],
        active: false,
    };

    println!("name: {}", user_one.name);
    println!("sign-in count: {}", user_one.sign_in_count);
    println!("affliation: {:?}", user_one.affliation);
    println!("is active? {}", user_one.active);

    // =================================

    // let mut user_two: User;
    // user_two = User{
    //     name: String::from("Varian Wrynn"),
    //     sign_in_count: 12,
    //     affliation: vec![
    //         String::from("High King of the Alliance"),
    //         String::from("King of Stormwind"),
    //         String::from("Champion of the Crimson Ring"),
    //     ],
    //     active: false,
    // };

    // user_two.name = String::from("Anduin Wrynn");
    // user_two.affliation.pop();
    // user_two.active = true;

    // println!("name: {}", user_two.name);
    // println!("sign-in count: {}", user_two.sign_in_count);
    // println!("affliation: {:?}", user_two.affliation);
    // println!("is active? {}", user_two.active);

    // =================================

    // let car_one = Car{
    //     brand: String::from("Toyota"),
    //     model: String::from("Sprinter Trueno AE86"),
    // };
    // println!("{} {}", car_one.brand, car_one.model);

    // let car_two: Car = Car{
    //     brand: String::from("BMW"),
    //     model: String::from("M3 GTR"),
    // };
    // println!("{} {}", car_two.brand, car_two.model);

    // let mut car_three: Car;
    // car_three = Car{
    //     brand: String::from("Audi"),
    //     model: String::from("Le Mans Quattro"),
    // };
    // println!("{} {}", car_three.brand, car_three.model);

    // let mut car_four: Car;
    // car_four = Car{
    //     brand: String::from("Audi Brand"),
    //     ..car_three
    // };
    // println!("{} {}", car_four.brand, car_four.model);

    // let model = String::from("Corvette C1");
    // let car_five = Car{
    //     brand: String::from("Chevrolet"),
    //     model,
    // };
    // println!("{} {}", car_five.brand, car_five.model);

    // let car_six = new_car(String::from("Chevrolet"), String::from("Corvette C6"));
    // println!("{} {}", car_six.brand, car_six.model);

    // =================================

    // struct Point {
    //     x: f32,
    //     y: f32,
    // }
    // let point_one = Point { x: 3.14, y: 8.0 };
    // println!("x: {}, y: {}", point_one.x, point_one.y);

    // let Point { x: x_one, y: y_one } = point_one;
    // println!("x_one: {}", x_one);
    // println!("y_one: {}", y_one);

    // =================================

    // #[derive(Debug)]
    // struct StructOne;
    // let data_struct_one = StructOne;
    // println!("data_struct_one: {:#?}", data_struct_one);

    // =================================

    // #[derive(Debug)]
    // struct GamingConsole {
    //     name: String
    // }
    // let console_one = GamingConsole{
    //     name: String::from("PlayStation 5"),
    // };
    // println!("data_struct_one: {:#?}", console_one);

    // =================================

    // struct Color(i32, i32, i32);
    // let red = Color(255, 0, 0);
    // println!("{:?} {:?} {:?}", red.0, red.1, red.2);

    // struct SomeTupleStruct(i32, bool);
    // let some_data = SomeTupleStruct(0, false);
    // println!("{:?} {:?}", some_data.0, some_data.1);

}

fn new_car(brand: String, model: String) -> Car {
    Car{
        brand,
        model,
    }
}