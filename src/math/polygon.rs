
pub trait Collider {
    type ReturnType;
    fn intersects_with(&self, obj: &Self) -> Option<Self::ReturnType>;
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub enum Axis {
    x,
    y,
    Custom { line: Line }
}

impl<'a> Point {
    fn min(&'a self, point: &'a Self, axis: Axis) -> &'a Point {
        match axis {
            Axis::x => {
                match f32::max(self.x, point.x) == self.x {
                    
                }
            },
        }
    }
}

pub struct Line {
    pub a: Box<Point>,
    pub b: Box<Point>,
}

impl Line {
    pub fn slope(&self) -> f32 {
        (self.b.y - self.a.y) / (self.b.x - self.a.x)
    }

    pub fn f(&self, x: f32) -> Option<f32> {
        let min_point = f32::min(self.a.x, self.b.x);
        let max_point = f32::max(self.a.x, self.b.x);

        if x < min_point || x > max_point {
            return None;
        }

        Some((self.slope() * x) + min_point)
    }
}

impl Collider for Line {
    type ReturnType = (Point, Point);
    fn intersects_with(&self, line: &Line) -> Option<(Point, Point)> {
        let a: &Box<Point> = &self.a;
        let b: &Box<Point> = &self.b;
        let c: &Box<Point> = &line.a;
        let d: &Box<Point> = &line.b;

        let farLeftPoint: Point = f32::min(self, other)

        let e: Point = Point { x: c.x, y: self.f(a.x)?};
        let f: Point = Point { x: d.x, y: self.f(b.x)?};

        if e.y > c.y && f.y < d.y
        || e.y < c.y && f.y > d.y {
            return Some((e, f)); 
        }

        None
    } 
}

pub struct Polygon {
    pub l: Box<Line>,
    pub m: Box<Line>,
    pub n: Box<Line>
}