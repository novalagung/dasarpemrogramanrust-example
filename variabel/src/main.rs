fn main() {
    // let nama_variabel = "predefined value";
    // println!(nama_variabel); // <---- error
    
    let nama_variabel = "predefined value";
    println!("{}", nama_variabel);

    let mut message_number = 1;
    let message1 = "hello";
    println!("message number {}: {}", message_number, message1);

    message_number = 2;
    let message2 = "world";
    println!("message number {}: {}", message_number, message2);

    message_number = 3;
    let message3: i8 = 24;
    println!("message number {1}: {0}", message3, message_number);

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

    let data1 = 24i8;
    println!("data1: {0}", data1);

    let x = 5;
    println!("x: {}", x);

    let x = x + 1;
    println!("x: {}", x);
}
