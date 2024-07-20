pub struct Square {
    pub side: i64,
}

impl crate::shape::Shape for Square {
    type Area = i64;

    fn area(&self) -> Self::Area {
        self.side * self.side
    }
}
