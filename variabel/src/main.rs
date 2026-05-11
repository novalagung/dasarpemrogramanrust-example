fn main() {
    // A.4.1 - error: println with variable (not string literal)
    // let nama_variabel = "predefined value";
    // println!(nama_variabel); // <---- error

    // A.4.1 - fixed with formatted print
    let nama_variabel = "predefined value";
    println!("{}", nama_variabel);

    // A.4.2 - immutable variables (working)
    let message_number = 1;
    let message1 = "hello";
    println!("message number {}: {}", message_number, message1);

    // A.4.2 - error: reassignment to immutable variable
    // message_number = 2;
    // let message2 = "world";
    // println!("message number {}: {}", message_number, message2);

    // A.4.3 - mutable variable (fix for the error above)
    let mut message_number = 1;
    let message1 = "hello";
    println!("message number {}: {}", message_number, message1);

    message_number = 2;
    let message2 = "world";
    println!("message number {}: {}", message_number, message2);

    // A.4.3 - more println format
    message_number = 3;
    let message3: i8 = 24;
    println!("message number {1}: {0}", message3, message_number);

    // A.4.3 - println format variants (same output)
    println!("message number {}: {}", message_number, message3);
    println!("message number {0}: {1}", message_number, message3);
    println!("message number {1}: {0}", message3, message_number);

    // A.4.5 - declaration without predefined value
    let another_number: i32;
    another_number = 1;
    println!("message number {}", another_number);

    // A.4.6 - multiple variables in one statement
    let (var1, var2) = (24, "hello");
    println!("var1: {0}", var1);
    println!("var2: {0}", var2);

    let (var3, var4): (i8, i8) = (32, 12);
    println!("var3: {0}", var3);
    println!("var4: {0}", var4);

    let (var5, mut var6, var7): (i8, i8, i8) = (64, 12, 4);
    println!("var5: {0}", var5);
    println!("var6: {0}", var6);
    var6 = 24;
    println!("var6: {0}", var6);
    println!("var7: {0}", var7);

    // A.4.7 - type specified from value
    let data1 = 24i8;
    println!("data1: {0}", data1);

    // A.4.7 - type specified from value with underscore separator
    let data1 = 24_i8;
    println!("data1: {0}", data1);

    // A.4.8 - variable shadowing
    let x = 5;
    println!("x: {}", x);

    let x = x + 1;
    println!("x: {}", x);

    // A.4.9 - underscore variable to ignore unused value
    let _ = 42;
}
