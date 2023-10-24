//1 unit == 1 meter
#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32, 
    pub y: f32,
}


impl Vector2 {
    fn translate(vector: &Vector2, translation: Vector2, t: f32) -> Vector2 {
        Vector2 { 
            x: vector.x + translation.x, 
            y: vector.y + translation.y,
        }
    }

    fn set(&mut self, vector: Vector2) {
        *self = vector;
    }

    fn new(x: f32, y: f32) -> Vector2 {
        Self {
            x,
            y,
        }
    }
}