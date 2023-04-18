pub struct Wall {
    // Bottom points
    pub x1: i32,
    pub y1: i32,
    
    // Top points
    pub x2: i32,
    pub y2: i32,

    pub color: i32
}

impl Wall {
    pub fn new(_x1: i32, _y1: i32, _x2: i32, _y2: i32, _color: i32) -> Self {
        Wall {
            x1: _x1,
            y1: _y1,

            x2: _x2,
            y2: _y2,

            color: _color
        }
    }
}