pub struct Sector {
    // Wall start and end
    pub ws: i32, 
    pub we: i32, 

    // Height of top and bottom
    pub z1: i32, 
    pub z2: i32,

    // Top and bottom side colors
    pub ct: i32,  
    pub cb: i32,

    // Hold points on surface if there's something to draw
    pub surf_arr: [i32; 160],   // Screen width
    pub surface: i32,

    // Distance for drawing order
    pub dist: i32
}

impl Sector {
    pub fn new(_ws: i32, _we: i32, _z1: i32, _z2: i32, _ct: i32, _cb: i32) -> Self {
        Sector{
            ws: _ws,
            we: _we,

            z1: _z1,
            z2: _z2,

            ct: _ct,
            cb: _cb,

            surf_arr: [0; 160],
            surface: 0,

            dist: 0
        }
    }
}