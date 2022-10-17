pub fn read_entry() -> String {
    let mut message = std::string::String::new();
    let stdin_reader = std::io::stdin();
    let reader_res = stdin_reader.read_line(&mut message);

    if reader_res.is_err() {
        println!("error! {:?}", reader_res.err());
    }

    message.trim().to_string()
}
