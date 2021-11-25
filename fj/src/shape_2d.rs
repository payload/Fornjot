use crate::Shape;

/// A 2-dimensional shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape2d {
    /// A circle
    Circle(Circle),

    /// A difference between two shapes
    Difference(Box<Difference>),

    /// A rectangle
    Rectangle(Rectangle),
}

/// A circle
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Circle {
    /// The radius of the circle
    pub radius: f64,
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

/// A difference between two shapes
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Difference {
    /// The original shape
    pub a: Shape2d,

    /// The shape being subtracted
    pub b: Shape2d,
}

impl From<Difference> for Shape {
    fn from(shape: Difference) -> Self {
        Self::Shape2d(Shape2d::Difference(Box::new(shape)))
    }
}

impl From<Difference> for Shape2d {
    fn from(shape: Difference) -> Self {
        Self::Difference(Box::new(shape))
    }
}

/// A rectangle
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Rectangle {
    /// The size of the rectangle along the x-axis
    pub x: f64,

    /// The size of the rectangle along the y-axis
    pub y: f64,
}

impl From<Rectangle> for Shape {
    fn from(shape: Rectangle) -> Self {
        Self::Shape2d(Shape2d::Rectangle(shape))
    }
}

impl From<Rectangle> for Shape2d {
    fn from(shape: Rectangle) -> Self {
        Self::Rectangle(shape)
    }
}
