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