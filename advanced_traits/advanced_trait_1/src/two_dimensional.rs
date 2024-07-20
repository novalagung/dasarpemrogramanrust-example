pub struct Circle {
    pub radius: i32,
}

impl crate::calculation_spec::Area for Circle {
    fn calculate_area(&self) -> f64 {
        3.14 * (self.radius.pow(2) as f64)
    }
}

pub struct Square {
    pub length: i32,
}

impl crate::calculation_spec::Area for Square {
    fn calculate_area(&self) -> f64 {
        self.length.pow(2) as f64
    }
}
