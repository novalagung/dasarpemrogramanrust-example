// A.25.1, A.25.2, A.25.3, A.25.4 - struct Car & methods
#[derive(Debug)]
pub struct Car {
    brand: String,
    model: String,
    manufacture_year: i32,
}

impl Car {
    // A.25.2 - associated function new
    pub fn new(brand: String, model: String) -> Self {
        Self { brand, model, manufacture_year: 0 }
    }

    // A.25.2 - method info
    pub fn info(&self) -> String {
        if self.manufacture_year == 0 {
            format!("{} model {}", self.brand, self.model)
        } else {
            format!(
                "{} model {}, manufactured at {}",
                self.brand,
                self.model,
                self.manufacture_year
            )
        }
    }

    // A.25.3 - method dengan parameter
    pub fn congratulate(&self, name: String) {
        println!("hello {}", name);
        println!("congrats with your new car {}", self.info());
        println!("vroooom vroooooooommmmm!");
    }

    // A.25.4 - method mutability
    pub fn set_manufacture_year(&mut self, year: i32) {
        self.manufacture_year = year
    }
}
