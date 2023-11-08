pub struct Vector2 {
    x: u16,
    y: u16
}

impl Vector2 {
    pub fn new(x: u16, y: u16) -> Vector2 {
        Vector2 { 
            x,
            y 
        }
    }
}