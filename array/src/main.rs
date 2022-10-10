fn main() {
    let mut numbers = [24, 12, 32, 7];
    println!("array {:?}", numbers);

    let data0 = numbers[0];
    println!("elemen array ke 0 {data0}");
    
    let data1 = numbers[1];
    println!("elemen array ke 1 {data1}");
    
    numbers[1] = 16;
    numbers[3] = 8;
    println!("array {numbers:?}");

    // -----------------------

    // let angka_integer = [24, 12, 32, 7];
    // println!("{angka_integer:?}");
    // // output: [24, 12, 32, 7]

    // let angka_float = [24.2, 12.5, 32.00002, 7.2];
    // println!("{angka_float:?}");
    // // output: [24.2, 12.5, 32.00002, 7.2]

    // -----------------------

    // let data_boolean: [bool; 2] = [false, true];
    // println!("{data_boolean:?}");
    // // output: [false, true]

    // let angka_unsigned_integer: [u32; 3] = [24, 0, 12];
    // println!("{angka_unsigned_integer:?}");
    // // output: [24, 0, 12]

    // -----------------------

    // let data_numerik1: [i32; 10] = [0; 10];
    // println!("{data_numerik1:?}");
    // // output: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    // let data_numerik2 = [4; 5];
    // println!("{data_numerik2:?}");
    // // output: [4, 4, 4, 4, 4]

    // -----------------------

    // let names = ["jason", "grayon", "drake", "damian"];
    // let length = names.len();
    // println!("array size is {}", length);

    // -----------------------

    // let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];
    // for name in names {
    //     println!("{name}");
    // }

    // -----------------------

    // let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];
    // for i in 0..names.len() {
    //     println!("array index ke-{}: {}", i, names[i]);
    // }

    // -----------------------

    // let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];

    // let mut i = 0;
    // while i < names.len() {
    //     println!("array index ke-{}: {}", i, names[i]);
    //     i += 1;
    // }

    // -----------------------

    // let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];

    // let mut i = 0;
    // loop {
    //     if i >= names.len() {
    //         break;
    //     }

    //     println!("array index ke-{}: {}", i, names[i]);
    //     i += 1;
    // }

    // -----------------------

    // let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];

    // for (i, name) in names.iter().enumerate() {
    //     println!("array index ke-{i}: {name}");
    // }

    // -----------------------

    // let data_arr = [
    //     ["salad", "fried rice"],
    //     ["apple", "coconut"],
    //     ["spinach", "jalapeno"],
    // ];
    // for sub_arr in data_arr {
    //     for el in sub_arr {
    //         print!("{el}, ");
    //     }
    //     println!();
    // }
}
