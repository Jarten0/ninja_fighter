pub trait Collider {
    fn intersects_with(&self, obj: Self) -> Option<Point>;
}

pub struct Point {
    x: f32,
    y: f32,
}

pub struct Line {
    a: Box<Point>,
    b: Box<Point>,
}