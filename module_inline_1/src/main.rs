mod utilities {

    pub mod random {

        pub fn string(length: u32) -> String {
            use rand::Rng;
            
            const CHARSET: &[u8] = "abcdefghijklmnopqrstuvwxyz".as_bytes();
            let mut arr = Vec::new();
            for _ in 0..=length {
                let n = rand::thread_rng().gen_range(0..(CHARSET.len()));
                let char = CHARSET[n];
                arr.push(char);
            }

            std::str::from_utf8(&arr[..]).unwrap().to_string()
        }
    }

    pub mod password {

        pub fn hash(text: &str) -> String {
            let result = bcrypt::hash(text, bcrypt::DEFAULT_COST).unwrap();
            result
        }

        pub fn is_valid(plain: &str, hashed: &str) -> bool {
            let valid = bcrypt::verify(plain, hashed).unwrap();
            valid
        }
    }
}

fn main() {
    let password = format!("zereth mortis {}", utilities::random::string(10));
    println!("raw password: {}", password);

    let hashed = utilities::password::hash(&password);
    println!("hashed password: {}", hashed);

    let is_valid = utilities::password::is_valid(&password, &hashed);
    println!("is password matched? {}", is_valid);
}
