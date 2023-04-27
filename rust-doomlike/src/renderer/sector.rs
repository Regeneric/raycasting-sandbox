use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Sector {
    // Wall start and end
    pub ws: i32, 
    pub we: i32, 

    // Height of top and bottom
    pub z1: i32, 
    pub z2: i32,

    // Hold points on surface if there's something to draw
    pub surf_arr: Vec<i32>,
    pub surface: i32,
    
    pub surface_texture: i32,
    pub texture_scale: i32,

    // Distance for drawing order
    pub dist: i32
}