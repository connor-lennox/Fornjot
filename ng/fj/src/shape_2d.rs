use crate::Shape;

/// A 2-dimensional shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape2d {
    /// A circle
    Circle(Circle),

    /// A square
    Square(Square),
}

/// A circle
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Circle {
    /// The radius of the circle
    pub radius: f32,
}

impl From<Circle> for Shape {
    fn from(shape: Circle) -> Self {
        Self::Shape2d(Shape2d::Circle(shape))
    }
}

impl From<Circle> for Shape2d {
    fn from(shape: Circle) -> Self {
        Self::Circle(shape)
    }
}

/// A square
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Square {
    /// The side length of the square
    pub size: f32,
}

impl From<Square> for Shape {
    fn from(shape: Square) -> Self {
        Self::Shape2d(Shape2d::Square(shape))
    }
}

impl From<Square> for Shape2d {
    fn from(shape: Square) -> Self {
        Self::Square(shape)
    }
}