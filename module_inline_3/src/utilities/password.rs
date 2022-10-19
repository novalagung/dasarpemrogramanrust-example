pub fn hash(text: &str) -> String {
    let result = bcrypt::hash(text, bcrypt::DEFAULT_COST).unwrap();
    result
}

pub fn is_valid(plain: &str, hashed: &str) -> bool {
    let valid = bcrypt::verify(plain, hashed).unwrap();
    valid
}
