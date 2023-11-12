
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { 
            x,
            y 
        }
    }

    // Returns a new interpolated vector.
    pub fn lerp_new(one: Vector2, other: Vector2,d: f32) -> Vector2 {
        let x = (one.x + (other.x - one.x)*d);
        let y = (one.y + (other.y - one.y)*d);

        Vector2 {
            x,
            y
        }
    }
}

impl Vector2 {
    /// Lerps using self as the starting and other as ending. 0 < d < 1
    /// Modifies original vec
    pub fn lerp(&mut self, other: &Vector2,d: f32) {
        let x = (self.x + (other.x - self.x)) * d;
        let y = (self.y + (other.y - self.y)) * d;

        self.x = x;
        self.y = y;
    }

    /// Degrees
    pub fn lookat_angle(&self, other: &Vector2) -> f32 {
        
        let dy = other.y - self.y;
        let dx = other.x - self.x;

        (dy).atan2(dx).to_degrees() + 90.0
    } 

    /// The Euclidean distance between two vectors.
    pub fn distance(&self, other: &Vector2) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
}