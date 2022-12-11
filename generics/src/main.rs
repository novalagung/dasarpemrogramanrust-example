fn main() {
    do_something::<bool>(24, false);
}

fn do_something<T>(arg1: i32, arg2: T) {
    println!("{:?}", arg2)
    // ...
}

// fn print_x_times<T>(data: T, x: i32) {
//     for _ in 0..x {
//         println!("{:?}", data);
//     }
//     let mut sum = 0;
//     for item in list {
//         sum += item;
//     }

//     largest
// }

// fn averages<T>(list: &[T]) -> T {
//     let mut sum: T = 0;
//     for item in list {
//         sum += item;
//     }

//     largest
// }


// fn average<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
