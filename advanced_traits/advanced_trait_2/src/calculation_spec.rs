pub trait Area {
    fn calculate_area(&self) -> f64;
}

// A.37.3 - Menambahkan trait Circumference
pub trait Circumference {
    fn calculate_circumference(&self) -> f64;
}