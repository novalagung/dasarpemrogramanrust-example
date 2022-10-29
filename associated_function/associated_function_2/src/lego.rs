#[derive(Debug)]
pub struct LegoSet {
    pub code: i32,
    pub name: String,
    pub category: String,
    pub age_minimum: i32,
}

impl LegoSet {

    pub fn new(code: i32, name: String, category: String, age_minimum: i32) -> Self {
        Self { code, name, category, age_minimum }
    }
}
