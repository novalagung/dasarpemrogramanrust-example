// A.37.7 - Associated types pada trait
pub trait Shape {
    type Area;

    fn area(&self) -> Self::Area;
}
