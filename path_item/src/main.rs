fn main() {
    // tampilkan intro untuk user agar menginput sebuah pesan
    println!("enter a message:");

    // variabel yang akan menampung inputan user dalam string
    let mut message = std::string::String::new();

    // objek reader untuk membaca inputan user
    let stdin_reader = std::io::stdin();

    // proses pembacaan inputan user
    let reader_res = stdin_reader.read_line(&mut message);

    // pengecekan apakah ada error dalam pembacaan inputan.
    // jika iya, maka tampilkan error dan hentikan program
    if reader_res.is_err() {
        println!("error! {:?}", reader_res.err());
        return;
    }

    // tampilkan pesan inputan user
    println!("message: {}", message);
}