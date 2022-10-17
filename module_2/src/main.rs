// definisi module my_io
mod my_io;

// definisi module my_number
mod my_number;

// fungsi main
fn main() {

    // tampilkan pesan untuk user agar menginput angka
    println!("enter any number:");

    // baca kemudian tampilkan inputan user
    let message = my_io::read_entry();
    println!("your number: {}", message);

    // convert inputan ke bentuk numerik i32
    let number = my_number::string_to_number(message);
    
    // cek apakah angka adalah ganjil, kemudian tampilkan
    let result = my_number::is_odd_number(number);
    println!("is odd number: {}", result);
}
