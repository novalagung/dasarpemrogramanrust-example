fn main() {
    // A.13.1 - pengenalan array
    let mut numbers = [24, 12, 32, 7];
    println!("array {:?}", numbers);

    let data0 = numbers[0];
    println!("elemen array ke 0 {data0}");

    let data1 = numbers[1];
    println!("elemen array ke 1 {data1}");

    numbers[1] = 16;
    numbers[3] = 8;
    println!("array {numbers:?}");

    // A.13.1 - deklarasi array (multiline & inline)
    let mut alphabets = [
        "a",
        "b",
        "c",
        "d"
    ];
    let booleans = [
        true,
        false
    ];
    let floatingNumbers = [32.0000078, 3.14, 0.5];

    println!("{alphabets:?} {booleans:?} {floatingNumbers:?}");

    // A.13.1 - formatted print {:?} dan {var:?}
    println!("array {:?}", numbers);
    println!("array {numbers:?}");

    // A.13.3 - type inference
    let angka_integer = [24, 12, 32, 7];
    println!("{angka_integer:?}");

    let angka_float = [24.2, 12.5, 32.00002, 7.2];
    println!("{angka_float:?}");

    // A.13.3 - manifest typing
    let data_boolean: [bool; 2] = [false, true];
    println!("{data_boolean:?}");

    let angka_unsigned_integer: [u32; 3] = [24, 0, 12];
    println!("{angka_unsigned_integer:?}");

    // A.13.3 - notasi [T; N]
    let data_numerik1: [i32; 10] = [0; 10];
    println!("{data_numerik1:?}");

    let data_numerik2 = [4; 5];
    println!("{data_numerik2:?}");

    // A.13.4 - method len
    let names = ["jason", "grayon", "drake", "damian"];
    let length = names.len();
    println!("array size is {}", length);

    // A.13.5 - for in array
    let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];
    for name in names {
        println!("{name}");
    }

    // A.13.5 - for in range
    let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];
    for i in 0..names.len() {
        println!("array index ke-{}: {}", i, names[i]);
    }

    // A.13.6 - while
    let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];

    let mut i = 0;
    while i < names.len() {
        println!("array index ke-{}: {}", i, names[i]);
        i += 1;
    }

    // A.13.6 - loop
    let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];

    let mut i = 0;
    loop {
        if i >= names.len() {
            break;
        }

        println!("array index ke-{}: {}", i, names[i]);
        i += 1;
    }

    // A.13.7 - for in enumerate
    let names: [&str; 4] = ["jason", "grayon", "drake", "damian"];

    for (i, name) in names.iter().enumerate() {
        println!("array index ke-{i}: {name}");
    }

    // A.13.9 - nested array
    let data_arr = [
        ["salad", "fried rice"],
        ["apple", "coconut"],
        ["spinach", "jalapeno"],
    ];
    for sub_arr in data_arr {
        for el in sub_arr {
            print!("{el}, ");
        }
        println!();
    }
}
