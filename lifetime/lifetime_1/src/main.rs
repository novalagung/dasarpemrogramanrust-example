fn main() {
    // A.44.2 - print_message: parameter reference (valid)
    let message = String::from("darkspear is better than zandalari");
    print_message(&message);

    // A.44.2 - dangling reference error demo dengan &String
    // {
    //     fn get_message() -> &String {
    //         let message = String::from("darkspear is better than zandalari");
    //         &message
    //     }
    //     let m: &String = get_message();
    //     println!("the message: {m}");
    // }

    // A.44.2 - Solusi: return String instead of &String
    let m = get_message();
    println!("the message: {m}");

    // A.44.2 - dangling reference error demo dengan &i32
    // {
    //     fn get_number() -> &i32 {
    //         let number = 13;
    //         &number
    //     }
    //     let n = get_number();
    //     println!("the number: {n}");
    // }

    // A.44.2 - Error: return &13 tanpa lifetime
    // fn get_number() -> &i32 {
    //     &13
    // }

    // A.44.2 - Solusi dengan lifetime 'static
    // fn get_number() -> &'static i32 {
    //     &13
    // }

    // A.44.4 - Lifetime annotation
    let n = get_number();
    println!("the number: {n}");
}

// A.44.2 - Fungsi dengan reference parameter
fn print_message(m: &String) {
    println!("the message: {m}")
}

// A.44.2 - Solusi: return String (move semantics)
fn get_message() -> String {
    let message = String::from("darkspear is better than zandalari");
    message
}

// A.44.4 - Fungsi dengan lifetime annotation
fn get_number<'my_lifetime>() -> &'my_lifetime i32 {
    &13
}
