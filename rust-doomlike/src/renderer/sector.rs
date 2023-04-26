use serde::Deserialize;

use super::WIDTH;
use super::RENDER_W;

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

impl Sector {
    pub fn new() -> Self {
        // Data is loaded from JSON file
        Sector{
            ws: 0,
            we: 0,

            z1: 0,
            z2: 0,

            surf_arr: vec![0; RENDER_W as usize],
            surface: 0,

            surface_texture: 0,
            texture_scale: 0,

            dist: 0
        }
    }
}