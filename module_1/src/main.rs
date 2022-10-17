// definisi module my_io
mod my_io;

// fungsi main
fn main() {

    // tampilkan pesan untuk user agar menginput angka
    println!("enter any number:");

    // baca kemudian tampilkan inputan user
    let message = my_io::read_entry();
    println!("your number: {}", message);
}
