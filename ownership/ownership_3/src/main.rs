fn main() {
    // A.34.6 - Transfer ownership
    let msg1 = String::from("hello");
    let msg2 = msg1;
    let msg3 = msg2;
    println!("{:?}", msg3);

    // Error: msg2 was moved into msg3
    // let msg4 = msg2;
    // println!("{:?}", msg4);
}
