// A.20.2 - definisi module my_io
// definisi module my_io
mod my_io;

// fungsi main
fn main() {

    // A.20.2 - praktik normal module (nama_module.rs)
    // tampilkan pesan untuk user agar menginput angka
    println!("enter any number:");

    // baca kemudian tampilkan inputan user
    let message = my_io::read_entry();
    println!("your number: {}", message);
}
