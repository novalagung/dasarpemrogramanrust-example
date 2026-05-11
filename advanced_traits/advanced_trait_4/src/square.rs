// A.37.7 - Struct Square dengan implementasi Shape (associated type Area = i64)
pub struct Square {
    pub side: i64,
}

impl crate::shape::Shape for Square {
    type Area = i64;

    fn area(&self) -> Self::Area {
        self.side * self.side
    }
}
