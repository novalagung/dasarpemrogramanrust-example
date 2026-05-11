// A.37.7 - Struct Circle dengan implementasi Shape (associated type Area = f64)
pub struct Circle {
    pub radius: f64,
}

impl crate::shape::Shape for Circle {
    type Area = f64;

    fn area(&self) -> Self::Area {
        std::f64::consts::PI * self.radius * self.radius
    }
}
