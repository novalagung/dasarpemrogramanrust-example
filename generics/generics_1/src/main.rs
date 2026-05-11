fn main() {
    // A.38.1 - Generics basic
    do_something::<bool>(24, false);
    do_something(24, false);

    // A.38.2 - Mengasosiasikan traits ke parameter generic (contoh ke-1)
    print_x_times("Hello guys", 10);

    // A.38.2 - Mengasosiasikan traits ke parameter generic (contoh ke-2)
    let data_arr = [0, 1, 2, 3];
    let largest_number1 = find_largest_number(&data_arr);
    println!("largest_number1: {:?}", largest_number1);

    let data_vec = vec![4, 5, 6, 7];
    let largest_number2 = find_largest_number(&data_vec);
    println!("largest_number2: {:?}", largest_number2);

    // A.38.3 / A.38.4 - Multi traits / keyword where
    print_largest_number(&[0, 1, 2, 3]);
}

// A.38.1 - Fungsi dengan parameter generic T
fn do_something<T>(arg1: i32, arg2: T) {
    // ...
}

// A.38.1 - Fungsi dengan 2 parameter generic R dan T
fn do_something_v2<R, T>(arg1: R, arg2: T) {
    // ...
}

// A.38.2 - Contoh ke-1: print data T sebanyak x kali
fn print_x_times<T: std::fmt::Debug>(data: T, x: i32) {
    for _ in 0..x {
        println!("{:?}", data);
    }
}

// A.38.2 - Contoh ke-2: mencari nilai maksimum dari slice
fn find_largest_number<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// A.38.3 / A.38.4 - Multi traits dan keyword where
fn print_largest_number<T>(list: &[T])
where
    T: std::cmp::PartialOrd + std::fmt::Debug,
{
    let largest = find_largest_number::<T>(list);
    println!("largest number: {:?}", largest);
}

// A.38.2 - Contoh ke-3: custom trait
fn do_something_v3<T: MyTrait>(arg1: T) {
    // do something
}

trait MyTrait {
    // methods declaration
}
