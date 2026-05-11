// A.37.1 - Struct Circle dengan implementasi trait Area
pub struct Circle {
    pub radius: i32,
}

impl crate::calculation_spec::Area for Circle {
    fn calculate_area(&self) -> f64 {
        3.14 * (self.radius.pow(2) as f64)
    }
}

// A.37.1 - Struct Square dengan implementasi trait Area
pub struct Square {
    pub length: i32,
}

impl crate::calculation_spec::Area for Square {
    fn calculate_area(&self) -> f64 {
        self.length.pow(2) as f64
    }
}
