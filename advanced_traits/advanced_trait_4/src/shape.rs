pub trait Shape {
    type Area;

    fn area(&self) -> Self::Area;
}
