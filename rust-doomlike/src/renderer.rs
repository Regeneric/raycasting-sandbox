pub mod player;
use std::mem::swap;

pub use crate::renderer::player::Player;

mod wall;
use crate::renderer::wall::Wall;

mod sector;
use crate::renderer::sector::Sector;


use sfml::{
    graphics::{RenderWindow, Color, Vertex, VertexBufferUsage, PrimitiveType, VertexBuffer, RenderTarget}, 
    system::{Vector2f},
};


pub struct Renderer {
    walls: Vec<Wall>,
    sectors: Vec<Sector>
}
impl Renderer {
    pub fn new() -> Self {
        let (sb, wb) = Self::data_loader();

        Renderer{
            walls: wb,
            sectors: sb,
        }
    }
 
    fn data_loader() -> (Vec<Sector>, Vec<Wall>) {
        // Prototype of data loader - it will be working on files in the near future

        // WS , WE  ;  Z1 , Z2  ;  Top color , Bottom color
        let sectors_data: Vec<i32> = Vec::from([
            0,   4, 0, 40, 2, 3,
            4,   8, 0, 50, 4, 5,
            8,  12, 0, 40, 6, 7,
            12, 16, 0, 50, 0, 1,
        ]);
        
        // X1, Y1  ;  X2, Y2  ;  COLOR
        let walls_data: Vec<i32> = Vec::from([
             0,  0, 32,  0, 0,
            32,  0, 32, 32, 1,
            32, 32,  0, 32, 0,
             0, 32,  0,  0, 1,

            64,  0, 96,  0, 2,
            96,  0, 96, 32, 3,
            96, 32, 64, 32, 2,
            64, 32, 64,  0, 3,

            64, 64, 96, 64, 4,
            96, 64, 96, 96, 5,
            96, 96, 64, 96, 4,
            64, 96, 64, 64, 5,

             0, 64, 32, 64, 6,
            32, 64, 32, 96, 7,
            32, 96,  0, 96, 6,
             0, 96,  0, 64, 7,
        ]);
        
        // // WS , WE  ;  Z1 , Z2  ;  Top color , Bottom color
        // let sectors_data: Vec<i32> = Vec::from([
        //     0,   4, 0, 40, 2, 3,
        //     4,   8, 0, 40, 4, 5,
        //     8,  12, 0, 40, 6, 7,
        //     12, 16, 0, 50, 0, 1,
        //     16, 22, 0,  6, 1, 0,
        // ]);
        
        // // X1, Y1  ;  X2, Y2  ;  COLOR
        // let walls_data: Vec<i32> = Vec::from([
        //     88, 120, 96, 120, 2,
        //     96, 120, 96, 208, 3,
        //     96, 208, 88, 208, 2,
        //     88, 208, 88, 120, 3,

        //     96 , 208, 144, 264, 6,
        //     144, 264, 136, 272, 7,
        //     136, 272, 88 , 216, 6,
        //     88 , 216, 96 , 208, 7,

        //     144, 264, 248, 264, 0,
        //     248, 264, 248, 272, 1,
        //     248, 264, 248, 272, 0,
        //     136, 272, 144, 264, 1,

        //     168, 120, 176, 120, 2,
        //     176, 120, 176, 232, 3,
        //     176, 232, 168, 232, 2,
        //     168, 232, 168, 120, 3,

        //     168, 133, 168, 208, 4,
        //     168, 208, 152, 208, 5,
        //     152, 208, 136, 160, 4,
        //     136, 192, 136, 192, 5, 
        //     136, 160, 151, 144, 4,
        //     152, 144, 168, 144, 5,
        // ]);


        let all_sectors = 4;
        let (mut v1, mut v2) = (0, 0);

        let mut sb: Vec<Sector> = Vec::new();
        let mut wb: Vec<Wall>   = Vec::new();

        for s in 0..all_sectors {
            // Adding new sector
            sb.push(Sector::new(sectors_data[v1+0],                         // WS
                                sectors_data[v1+1],                         // WE
                                sectors_data[v1+2],                         // Z2
                                sectors_data[v1+3]                          // Z1
                               -sectors_data[v1+2],
                                sectors_data[v1+4],                         // Top color
                                sectors_data[v1+5]));                       // Bottom color
            v1 = v1+6;

            for _w in sb[s].ws .. sb[s].we {
                // Adding walls to current sector
                wb.push(Wall::new(walls_data[v2+0],     // Bottom X1
                                  walls_data[v2+1],     // Bottom Y1
                                  walls_data[v2+2],     // Top X2
                                  walls_data[v2+3],     // Top Y2
                                  walls_data[v2+4]));   // Wall color
                v2 = v2+5;
            }
        }

        (sb, wb)
    }


    fn pixel(x: f32, y:  f32, color: Color,  draw: bool, window: &mut RenderWindow) -> Option<Vec<Vertex>> {        
        let mut pixels = VertexBuffer::new(PrimitiveType::QUADS, 4 as u32, VertexBufferUsage::STREAM);
        let mut verts: Vec<Vertex> = Vec::new();
            verts.push(Vertex::new(Vector2f::new(x      , y)      , color , Vector2f::new(x      , y)));
            verts.push(Vertex::new(Vector2f::new(x + 1.0, y)      , color , Vector2f::new(x + 1.0, y)));
            verts.push(Vertex::new(Vector2f::new(x + 1.0, y + 1.0), color , Vector2f::new(x + 1.0, y + 1.0)));
            verts.push(Vertex::new(Vector2f::new(x      , y + 1.0), color , Vector2f::new(x      , y + 1.0)));
        

        // If we wanna draw those quads, we can
        // But if there's a lot of them, we can return an array
        // and draw at once with some external VertexBuffer
        if draw {
            pixels.update(&verts, 0);
            window.draw(&pixels); 
            None
        } else {Some(verts)}
    }

    fn wall(&mut self, mut x1: i32, mut x2: i32,  b1: i32, b2: i32,  t1: i32, t2: i32,  c: i32,  s: i32,  window: &mut RenderWindow) {
        let wallpaint;
        match c {
            0 => wallpaint = Color::rgb(80 , 80 , 80),
            1 => wallpaint = Color::rgb(100, 100, 100),
            2 => wallpaint = Color::rgb(200, 0  , 0),
            3 => wallpaint = Color::rgb(230, 0  , 0),
            4 => wallpaint = Color::rgb(0  , 200, 0),
            5 => wallpaint = Color::rgb(0  , 230, 0),
            6 => wallpaint = Color::rgb(0  , 0  , 200),
            7 => wallpaint = Color::rgb(0  , 0  , 230),

            _ => wallpaint = Color::BLACK,
        }

        // let floors;
        // match self.sectors[s as usize].cb {
        //     3 => floors = Color::rgb(80 , 70 , 80),
        //     4 => floors = Color::rgb(100, 90 , 100),
        //     5 => floors = Color::rgb(190, 0  , 0),
        //     6 => floors = Color::rgb(210, 0  , 0),
        //     7 => floors = Color::rgb(0  , 190, 0),
        //     0 => floors = Color::rgb(0  , 220, 0),
        //     1 => floors = Color::rgb(0  , 0  , 190),
        //     2 => floors = Color::rgb(0  , 0  , 220),

        //     _ => floors = Color::BLACK,
        // }

        // let ceiling;
        // match self.sectors[s as usize].ct {
        //     7 => ceiling = Color::rgb(80 , 90 , 80),
        //     6 => ceiling = Color::rgb(100, 110, 100),
        //     5 => ceiling = Color::rgb(200, 10  , 0),
        //     4 => ceiling = Color::rgb(230, 10  , 0),
        //     3 => ceiling = Color::rgb(0  , 210, 0),
        //     2 => ceiling = Color::rgb(0  , 240, 0),
        //     1 => ceiling = Color::rgb(0  , 10  , 200),
        //     0 => ceiling = Color::rgb(0  , 10  , 230),

        //     _ => ceiling = Color::BLACK,
        // }


        let width  = 160;
        let height = 120;
        let mut verts: Vec<Vertex> = Vec::new();

        let delta_y_bottom = b2 - b1;
        let delta_y_top    = t2 - t1;

        let mut delta_x = x2-x1; if delta_x == 0 {delta_x = 1;}
        let starting_x = x1;

        // Don't draw behind camera
        if x1 < 1       {x1 = 1;}
        if x2 < 1       {x2 = 1;}
        if x1 > width-1 {x1 = width-1;}
        if x2 > width-1 {x2 = width-1;}
        
        for x in x1..x2 {
            let mut y1 = delta_y_bottom * (f32::floor((x - starting_x) as f32 + 0.5)) as i32 / delta_x + b1;
            let mut y2 = delta_y_top    * (f32::floor((x - starting_x) as f32 + 0.5)) as i32 / delta_x + t1;

            // Clip Y axis - don't draw where camera doesn't see
            if y1 < 1        {y1 = 1;}
            if y2 < 1        {y2 = 1;}
            if y1 > height-1 {y1 = height-1;}
            if y2 > height-1 {y2 = height-1;}


            // for sector in self.sectors.iter_mut() {
            //     if sector.surface == 1 {
            //         sector.surf_arr[x as usize] = y1;
            //     }
            //     if sector.surface == 2 {
            //         sector.surf_arr[x as usize] = y2;
            //     }

            //     if sector.surface == -1 {
            //         for y in (sector.surf_arr[x as usize]) .. y1 {
            //             verts.extend(Self::pixel(x as f32, y as f32, Color::WHITE, false, window).unwrap());
            //         }
            //     }
            //     if sector.surface == -2 {
            //         for y in y2 .. (sector.surf_arr[x as usize]) {
            //             verts.extend(Self::pixel(x as f32, y as f32, Color::WHITE, false, window).unwrap());
            //         }
            //     }
            // }

            // for y in y1..y2 {
            //     verts.extend(Self::pixel(x as f32, y as f32, wallpaint, false, window).unwrap());
            // }

            // Bottom points
            if self.sectors[s as usize].surface == 1 {
                self.sectors[s as usize].surf_arr[x as usize] = y1;
                continue;
            }
            // Top points
            if self.sectors[s as usize].surface == 2 {
                self.sectors[s as usize].surf_arr[x as usize] = y2;
                continue;
            }

            // Bottom side
            if self.sectors[s as usize].surface == -1 {
                for y in (self.sectors[s as usize].surf_arr[x as usize]) .. y1 {
                    verts.extend(Self::pixel(x as f32, y as f32, Color::WHITE, false, window).unwrap());
                }
            }
            // Top side
            if self.sectors[s as usize].surface == -2 {
                for y in y2 .. (self.sectors[s as usize].surf_arr[x as usize]) {
                    verts.extend(Self::pixel(x as f32, y as f32, Color::WHITE, false, window).unwrap());
                }
            }

            // Normal walls
            for y in y1..y2 {
                verts.extend(Self::pixel(x as f32, y as f32, wallpaint, false, window).unwrap());
            }
        }

        let mut pixels = VertexBuffer::new(PrimitiveType::QUADS, verts.len() as u32, VertexBufferUsage::STREAM);
        pixels.update(&verts, 0);
        window.draw(&pixels);
    }

    fn dist(x1: i32, y1: i32,  x2: i32, y2: i32) -> i32 {
        f32::floor(f32::sqrt(((x2-x1) as f32)*((x2-x1) as f32) + ((y2-y1) as f32)*((y2-y1) as f32))) as i32
    }

    fn clip_behind(x1: &mut i32, y1: &mut i32, z1: &mut i32,  x2: i32, y2: i32, z2: i32) {
        let distance_plane_pt_a = *y1 as f32;
        let distance_plane_pt_b =  y2 as f32;
        
        // let mut dist = distance_plane_pt_a - distance_plane_pt_b; if dist == 0.0 {dist = 1.0;}
        let intersection: f32 = distance_plane_pt_a / (distance_plane_pt_a - distance_plane_pt_b);

        *x1 = f32::floor((*x1 as f32) + intersection * ((x2 - (*x1)) as f32)) as i32;
        *y1 = f32::floor((*y1 as f32) + intersection * ((y2 - (*y1)) as f32)) as i32; if *y1 == 0 {*y1 = 1;}
        *z1 = f32::floor((*z1 as f32) + intersection * ((z2 - (*z1)) as f32)) as i32;
    }


    // Methods
    pub fn draw(&mut self, p: &Player, window: &mut RenderWindow) {
        let width  = 160;
        let height = 120;

        let mut wx: [i32; 4] = [0; 4];
        let mut wy: [i32; 4] = [0; 4];
        let mut wz: [i32; 4] = [0; 4];

        let cos: f32 = p.cos[usize::try_from(p.angle).unwrap()];
        let sin: f32 = p.sin[usize::try_from(p.angle).unwrap()];


        for s in 0..4 {
            for w in 0..(4-s-1) {
                if self.sectors[w].dist < self.sectors[w+1].dist {
                    self.sectors.swap(w, w+1);
                }
            }
        }


        for s in 0..self.sectors.len() {
            self.sectors[s].dist = 0;   // Clear distance (drawing order)
            
            if      p.pos.z < self.sectors[s].z1 {self.sectors[s].surface = 1;}     // Top
            else if p.pos.z > self.sectors[s].z2 {self.sectors[s].surface = 2;}     // Bottom
            else                                 {self.sectors[s].surface = 0;}

            for l in 0..2 {
                for w in self.sectors[s as usize].ws .. self.sectors[s as usize].we {        
                    // Offset bottom 2 point by player
                    let mut x1: i32 = self.walls[w as usize].x1 - p.pos.x; 
                    let mut y1: i32 = self.walls[w as usize].y1 - p.pos.y;
                    
                    let mut x2: i32 = self.walls[w as usize].x2 - p.pos.x; 
                    let mut y2: i32 = self.walls[w as usize].y2 - p.pos.y;
                    
                    // Drawing backfaces
                    if l == 0 {
                        swap(&mut x1, &mut x2);
                        swap(&mut y1, &mut y2);
                    }

    
                    // World X position
                    wx[0] = f32::floor((x1 as f32) * cos  -  (y1 as f32) * sin) as i32;
                    wx[1] = f32::floor((x2 as f32) * cos  -  (y2 as f32) * sin) as i32;
                    wx[2] = wx[0];
                    wx[3] = wx[1];
            
                    // World Y position
                    wy[0] = f32::floor((y1 as f32) * cos  +  (x1 as f32) * sin) as i32;
                    wy[1] = f32::floor((y2 as f32) * cos  +  (x2 as f32) * sin) as i32;   // Depth - how far wall is from the camera
                    wy[2] = wy[0];
                    wy[3] = wy[1];

                    // Walls distance - this is drawing order
                    self.sectors[s as usize].dist = f32::floor((self.sectors[s as usize].dist + Self::dist(0, 0,  (wx[0]+wx[1])/2,  (wy[0]+wy[1])/2)) as f32) as i32;
                    self.sectors[s as usize].surface = self.sectors[s as usize].surface * -1;   // Flip to negative - draw top or bottom surface
    
    
                    // World Z height
                    wz[0] = self.sectors[s as usize].z1 - p.pos.z + ((p.look_up_down * wy[0])/32);
                    wz[1] = self.sectors[s as usize].z1 - p.pos.z + ((p.look_up_down * wy[1])/32);
                    wz[2] = (wz[0] + self.sectors[s as usize].z2) - self.sectors[s as usize].z1;
                    wz[3] = (wz[1] + self.sectors[s as usize].z2) - self.sectors[s as usize].z1;
            
            
                    if wy[0] < 1 && wy[1] < 1 {continue;} // Wall behind player, don't draw it
                    if wy[0] < 1 {
                        let (mut x2, mut y2, mut z2) = (wx[1], wy[1], wz[1]);  
                        Self::clip_behind(&mut wx[0], &mut wy[0], &mut wz[0],  x2, y2, z2);
            
                        (x2, y2, z2) = (wx[3], wy[3], wz[3]);
                        Self::clip_behind(&mut wx[2], &mut wy[2], &mut wz[2],  x2, y2, z2);
                    } 
                    if wy[1] < 1 {
                        let (mut x2, mut y2, mut z2) = (wx[0], wy[0], wz[0]);  
                        Self::clip_behind(&mut wx[1], &mut wy[1], &mut wz[1],  x2, y2, z2);
            
                        (x2, y2, z2) = (wx[2], wy[2], wz[2]);
                        Self::clip_behind(&mut wx[3], &mut wy[3], &mut wz[3],  x2, y2, z2);
                    }
            
                    // Screen X and Y position
                    wx[0] = wx[0]*200 / wy[0]+width/2;  wy[0] = wz[0]*200 / wy[0]+height/2;
                    wx[1] = wx[1]*200 / wy[1]+width/2;  wy[1] = wz[1]*200 / wy[1]+height/2;
                    wx[2] = wx[2]*200 / wy[2]+width/2;  wy[2] = wz[2]*200 / wy[2]+height/2;
                    wx[3] = wx[3]*200 / wy[3]+width/2;  wy[3] = wz[3]*200 / wy[3]+height/2;
            
                    
                    let (x1, x2,  y1, y2, y3, y4) = (wx[0], wx[1],  wy[0], wy[1],  wy[2], wy[3]);
                    self.wall(x1, x2,  y1, y2,  y3, y4,  self.walls[w as usize].color, s as i32,  window);
                }
    
                self.sectors[s].dist = self.sectors[s as usize].dist  /  (self.sectors[s].we - self.sectors[s].ws); 
            }
        }
    }
}