fn main() {
    println!("hello rust");
    greet();

    // =======================

    // greet_custom_message("Damian", "welcome to the castle!");

    // =======================

    // let width = 5;
    // let height = 8;
    // let length = 12;

    // let res1 = calculate_box_volume1(width, height, length);
    // println!("result: {res1}");

    // let name2 = "Damian";
    // let res2 = calculate_box_volume2(width, height, length);
    // println!("hi {name2}, the box volume is {res2}");

    // let res3 = calculate_box_volume3(width, height, length);
    // let message3 = format!("the box volume is {}", res3);
    // greet_custom_message("Damian", message3.as_str());

    // =======================

    // println!("{}", get_score_message(100.0));
    // println!("{}", get_score_message(98.2));
    // println!("{}", get_score_message(33.12));

    // =======================

    // let a = greet_custom_message("Damian", "welcome to the castle!");
    // println!("result: {:?}", a);
}

fn greet() {
    println!("hello world")
}

fn greet_custom_message(name: &str, message: &str) {
    println!("hi {name}, {message}")
}

fn calculate_box_volume1(width: i32, height: i32, length: i32) -> i32 {
    let volume = width * height * length;
    return volume;
}

fn calculate_box_volume2(width: i32, height: i32, length: i32) -> i32 {
    let volume = width * height * length;
    volume
}

fn calculate_box_volume3(width: i32, height: i32, length: i32) -> i32 {
    width * height * length
}

fn get_score_message(score: f32) -> &'static str {
    if score == 100.0 {
        return "you got a perfect score!"
    }
    
    if score > 76.0 {
        return "congrats, you passed the exam!"
    }

    "your score is below the passing grade"
}