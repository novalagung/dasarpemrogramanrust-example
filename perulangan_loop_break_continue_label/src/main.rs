fn main() {
    let mut i = 0;

    loop {
        println!("nilai: {i}");
        i += 1;
    }
}

// -----------------------

// fn main() {
//     let mut i = 0;
//     let max = 5;

//     loop {
//         println!("nilai: {i}");
//         i += 1;
//         if i > max {
//             break;
//         }
//     }

//     println!("perulangan selesai");
// }

// -----------------------

// fn main() {
//     let mut i = 0;
//     let max = 5;

//     loop {
//         let mut j = max;
//         let max_inner = i;

//         loop {
//             print!("* ");
//             j -= 1;
//             if j < max_inner {
//                 break;
//             }
//         }
        
//         println!();

//         i += 1;
//         if i > max {
//             break;
//         }
//     }
// }

// -----------------------

// fn main() {
//     let mut i = 0;
//     let max = 15;

//     loop {
//         i += 1;

//         if i % 2 == 1 {
//             continue;
//         }

//         println!("nilai i: {i}");

//         if i > max {
//             break;
//         }
//     }
// }

// -----------------------

// fn main() {
//     let mut i = 0;
//     let max = 9;
    
//     'mainLoop: loop {
//         i += 1;
//         let mut j = 0;

//         loop {
//             if i > max {
//                 break 'mainLoop;
//             }

//             j += 1;
//             if j > i {
//                 break;
//             }

//             print!("{i} ");
//         }

//         println!();
//     }
// }

// -----------------------

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("result: {result}");
// }
