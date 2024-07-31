pub fn read_entry() -> String {
    // variabel yang akan menampung inputan user dalam string
    let mut message = std::string::String::new();

    // objek reader untuk membaca inputan user
    let stdin_reader = std::io::stdin();

    // proses pembacaan inputan user
    let reader_res = stdin_reader.read_line(&mut message);

    // pengecekan apakah ada error dalam pembacaan inputan
    if reader_res.is_err() {
        println!("error! {:?}", reader_res.err());
    }

    // trim whitespace dari message menggunakan .trim()
    // method tersebut menjadikan variabel bertipe data String menjadi &str.
    // chain dengan method .to_string() agar kembali ke tipe String 
    message.trim().to_string()
}
