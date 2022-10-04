fn main() {
    let tuple_a = ("jason", 27, ["racing", "working out"], true);
    println!("tuple_a: {:?}", tuple_a);
    
    println!("index 0: {:?}", tuple_a.0);
    println!("index 1: {:?}", tuple_a.1);
    println!("index 2: {:?} {:?}", tuple_a.2[0], tuple_a.2[1]);
    println!("index 3: {:?}", tuple_a.3);

    // ==========================

    // let mut tuple_b: (&str, i32, [&str; 2], bool) = ("", 0, [""; 2], false);
    // tuple_b.0 = "damian";
    // tuple_b.1 = 18;
    // tuple_b.2 = ["gaming", "adventuring"];
    // tuple_b.3 = true;

    // println!("tuple_b: {:?}", tuple_b);

    // ==========================

    // let name = "grayson";
    // let age = 29;
    // let hobbies = ["sleeping", "parkour"];

    // let tuple_c = (name, age, hobbies);
    
    // println!("name    : {:?}", tuple_c.0);
    // println!("age     : {:?}", tuple_c.1);
    // println!("hobbies : {:?}", tuple_c.2);

    // ==========================

    // let tuple_d = ("stephanie", 28, ["software engineering"], false);
    // let (name, age, hobbies, is_male) = tuple_d;
    
    // println!("name    : {:?}", name);
    // println!("age     : {:?}", age);
    // println!("hobbies : {:?}", hobbies);
    // println!("is_male : {:?}", is_male);

}
