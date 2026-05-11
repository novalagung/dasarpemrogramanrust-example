// A.37.1 - Struct Circle dengan implementasi Area
pub struct Circle {
    pub radius: i32,
}

impl crate::calculation_spec::Area for Circle {
    fn calculate_area(&self) -> f64 {
        3.14 * (self.radius.pow(2) as f64)
    }
}

// A.37.3 - Implementasi Circumference untuk Circle
impl crate::calculation_spec::Circumference for Circle {
    fn calculate_circumference(&self) -> f64 {
        2.0 * 3.14 * (self.radius) as f64
    }
}

// A.37.1 - Struct Square dengan implementasi Area
pub struct Square {
    pub length: i32,
}

impl crate::calculation_spec::Area for Square {
    fn calculate_area(&self) -> f64 {
        self.length.pow(2) as f64
    }
}

// A.37.3 - Implementasi Circumference untuk Square
impl crate::calculation_spec::Circumference for Square {
    fn calculate_circumference(&self) -> f64 {
        4.0 * (self.length) as f64
    }
}