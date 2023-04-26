use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Wall {
    // Bottom points
    pub x1: i32,
    pub y1: i32,
    
    // Top points
    pub x2: i32,
    pub y2: i32,

    pub texture: i32,
    
    // "U" and "V" denotes the axes of 2D texture (UV mapping)
    pub u: i32,
    pub v: i32,

    pub shade: u8
}

impl Wall {
    pub fn new() -> Self {
        // Data is loaded from JSON file
        Wall {
            x1: 0,
            y1: 0,

            x2: 0,
            y2: 0,

            texture: 0,
            u: 0,
            v: 0,

            shade: 0
        }
    }
}