use super::WIDTH;
use super::RENDER_W;

pub struct Texture {
    // Wall start and end
    pub width: i32, 
    pub height: i32, 

    pub name: String,
}

impl Texture {
    pub fn new(w: i32, h: i32, n: String) -> Self {
        Texture{
            width: w,
            height: h,
            name: n
        }
    }
}