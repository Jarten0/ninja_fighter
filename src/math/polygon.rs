pub trait Collider {
    type ReturnType;
    fn intersects_with(&self, obj: &Self) -> Option<Self::ReturnType>;
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Representation of a line traversing in a direction indicating an axis.
pub enum Axis {
    X,
    Y,
    Custom { line: Line },
}

#[allow(dead_code)]
impl<'a> Point {
    fn min_max(&'a self, point: &'a Self, axis: &Axis) -> (&'a Point, &'a Point) {
        (self.min(point, axis), self.max(point, axis))
    }

    /// Returns the point farthest in the direction in the given [`Axis`]
    ///
    /// If both points are equal, [`self`] (or the first point) is returned
    ///
    /// Custom axis is currently [`todo!`]
    fn max(&'a self, point: &'a Self, axis: &Axis) -> &'a Point {
        match axis {
            Axis::X => match f32::max(self.x, point.x) == self.x {
                true => {
                    return &self;
                }
                false => {
                    return &point;
                }
            },
            Axis::Y => match f32::max(self.y, point.y) == self.x {
                true => {
                    return &self;
                }
                false => {
                    return &point;
                }
            },
            Axis::Custom { line: _line } => {
                todo!()
            }
        }
    }

    /// Returns the point farthest in the opposite direction in the given [`Axis`]
    ///
    /// If both points are equal, [`self`] (or the first point) is returned
    ///
    /// Custom axis is currently [`todo!`]
    fn min(&'a self, point: &'a Self, axis: &Axis) -> &'a Point {
        match axis {
            Axis::X => match f32::min(self.x, point.x) == self.x {
                true => {
                    return &self;
                }
                false => {
                    return &point;
                }
            },
            Axis::Y => match f32::min(self.y, point.y) == self.x {
                true => {
                    return &self;
                }
                false => {
                    return &point;
                }
            },
            Axis::Custom { line: _line } => {
                todo!()
            }
        }
    }
}

pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn slope(&self) -> f32 {
        (self.b.y - self.a.y) / (self.b.x - self.a.x)
    }

    /// Returns the y-offset of the slope, or (b) in (y = mx + **b**)
    pub fn b(&self) -> f32 {
        self.f_unchecked(0.0)
    }

    pub fn f_unchecked(&self, x: f32) -> f32 {
        (self.slope() * x) + self.b()
    }

    /// Uses the standard slope formula (y = m**x** + b) and calculates for x.
    ///
    /// If the x value is outside of  
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

    #[allow(unused_variables)]
    fn intersects_with(&self, line: &Line) -> Option<(Point, Point)> {
        let a = &self.a;
        let b = &self.b;
        let c = &line.a;
        let d = &line.b;

        let (far_left_point, far_right_point) = Point::min_max(&self.a, &line.a, &Axis::X);

        let e: Point = Point {
            x: c.x,
            y: self.f(a.x)?,
        };
        let f: Point = Point {
            x: d.x,
            y: self.f(b.x)?,
        };

        if e.y > c.y && f.y < d.y || e.y < c.y && f.y > d.y {
            return Some((e, f));
        }

        None
    }
}

pub struct Polygon {
    pub l: Line,
    pub m: Line,
    pub n: Line,
}
