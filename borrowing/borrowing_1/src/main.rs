fn main() {
    // A.35.1 - Konsep borrowing
    // Error: move semantics
    // let msg_1 = String::from("hello");
    // let msg_2 = msg_1;
    // println!("{:?}", msg_2);
    // println!("{:?}", msg_1);

    // Borrowing dengan immutable reference
    {
        let msg_3 = String::from("hello rust");
        let msg_4 = &msg_3;
        println!("{:?}", msg_4);
        println!("{:?}", msg_3);
    }

    // Borrowing dengan mutable reference
    {
        let mut msg_3 = String::from("hello");
        let msg_4 = &mut msg_3;
        *msg_4 = String::from("hello rust");
        println!("{:?}", msg_4);
        println!("{:?}", msg_3);
    }

    // A.35.4 - Contoh ke-1: multiple immutable references
    {
        let msg_5 = String::from("hello rust");
        let msg_6 = &msg_5;
        let msg_7 = &msg_5;
        let msg_8 = &msg_5;
        println!("{:?} {:?} {:?}", msg_6, msg_7, msg_8);
    }

    // Contoh ke-2: single mutable reference
    {
        let mut msg_9 = String::from("hello rust");
        let msg_10 = &mut msg_9;
        println!("{:?}", msg_10);
    }

    // Contoh ke-2 (error): multiple mutable references
    // {
    //     let mut msg_9 = String::from("hello rust");
    //     let msg_10 = &mut msg_9;
    //     let msg_11 = &mut msg_9;
    //     println!("{:?} {:?}", msg_10, msg_11);
    // }

    // Contoh ke-3 (error): mutable + immutable references
    // {
    //     let mut msg_12 = String::from("hello rust");
    //     let msg_13 = &msg_12;
    //     let msg_14 = &mut msg_12;
    //     println!("{:?} {:?}", msg_13, msg_14);
    // }
}
