fn main() {
    // do_something::<bool>(24, false);
    // do_something(24, false);

    // ====================================

    // print_x_times("Hello guys", 10);
    
    // ====================================

    // let data_arr = [0, 1, 2, 3];
    // let largest_number1 = find_largest_number(&data_arr);
    // println!("largest_number1: {:?}", largest_number1);
    
    // let data_vec = vec![4, 5, 6, 7];
    // let largest_number2 = find_largest_number(&data_vec);
    // println!("largest_number2: {:?}", largest_number2);
    
    // ====================================

    print_largest_number(&[0, 1, 2, 3]);
}

fn do_something<T>(arg1: i32, arg2: T) {
    // ...
}

fn do_something_v2<R, T>(arg1: R, arg2: T) {
    // ...
}

fn print_x_times<T: std::fmt::Debug>(data: T, x: i32) {
    for _ in 0..x {
        println!("{:?}", data);
    }
}

fn find_largest_number<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn print_largest_number<T: std::cmp::PartialOrd + std::fmt::Debug>(list: &[T]) {
//     let largest = find_largest_number::<T>(list);
//     println!("largest number: {:?}", largest);
// }

fn print_largest_number<T>(list: &[T]) 
where
    T: std::cmp::PartialOrd + std::fmt::Debug,
    // parameter generic lainnya (jika diperlukan)
{
    let largest = find_largest_number::<T>(list);
    println!("largest number: {:?}", largest);
}

fn do_something_v3<T: MyTrait>(arg1: T) {
    // do something
}

trait MyTrait {
    // methods declaration
}