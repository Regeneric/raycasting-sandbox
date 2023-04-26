pub mod player;
use std::mem::swap;

pub use crate::renderer::player::Player;

mod wall;
use crate::renderer::wall::Wall;

mod sector;
use crate::renderer::sector::Sector;

mod level;
use crate::renderer::level::Level;

pub mod texture;
use crate::renderer::texture::Texture;

use super::{RENDER_W, RENDER_H, WIDTH, FOV};


use sfml::{
    graphics::{RenderWindow, Color, Vertex, VertexBufferUsage, PrimitiveType, VertexBuffer, RenderTarget}, 
    system::{Vector2f},
};



pub fn pixel(x: f32, y:  f32,  r: u8, g: u8, b: u8,  draw: bool, window: &mut RenderWindow) -> Option<Vec<Vertex>> {        
    let mut pixels = VertexBuffer::new(PrimitiveType::QUADS, 4 as u32, VertexBufferUsage::STREAM);
    let mut verts: Vec<Vertex> = Vec::new();
        verts.push(Vertex::new(Vector2f::new(x      , y)      , Color::rgb(r, g, b) , Vector2f::new(x      , y)));
        verts.push(Vertex::new(Vector2f::new(x + 1.0, y)      , Color::rgb(r, g, b) , Vector2f::new(x + 1.0, y)));
        verts.push(Vertex::new(Vector2f::new(x + 1.0, y + 1.0), Color::rgb(r, g, b) , Vector2f::new(x + 1.0, y + 1.0)));
        verts.push(Vertex::new(Vector2f::new(x      , y + 1.0), Color::rgb(r, g, b) , Vector2f::new(x      , y + 1.0)));
    

    // If we wanna draw those quads, we can
    // But if there's a lot of them, we can return an array
    // and draw at once with some external VertexBuffer
    if draw {
        pixels.update(&verts, 0);
        window.draw(&pixels); 
        None
    } else {Some(verts)}
}


pub struct Renderer {
    sectors: i32,
    sectors_data: Vec<Sector>,

    walls: i32,
    walls_data: Vec<Wall>,

    textures: Vec<Texture>,
}

impl Renderer {
    pub fn new() -> Self {
        // let level: Level = Self::data_loader().unwrap();
        let (w, s, sb, wb) = Level::level_loader();
        let tb = Texture::texture_loader();

        Renderer{
            sectors: s,
            walls: w,

            sectors_data: sb,
            walls_data: wb,

            textures: tb,
        }
    }

    pub fn floor(&mut self, p: &Player, window: &mut RenderWindow) {
        let offset_x = RENDER_W as i32 / 2;
        let offset_y = RENDER_H as i32 / 2;

        let mut look_up_down = -p.look_up_down as f32 * 2.0;
        if look_up_down > RENDER_H {look_up_down = RENDER_H}

        let mut move_up_down = p.pos.z as f32 / 16.0;
        if move_up_down == 0.0 {move_up_down = 0.001;}

        // let mut start_y = -look_up_down;
        // let mut end_y = offset_y as f32;

        // if move_up_down < 0.0 {start_y = offset_y as f32; end_y = offset_y as f32 + look_up_down;}

        let mut start_y = -offset_y as f32;
        let mut end_y = -look_up_down;

        if move_up_down > 0.0 {start_y = -look_up_down; end_y = offset_y as f32 + look_up_down;}

        for y in start_y as i32 .. end_y as i32 {
            for x in -offset_x..offset_x {
                let mut z = y as f32 + look_up_down;
                if z == 0.0 {z = 0.0001}

                let floor_x: f32 =   x as f32 / z * move_up_down;
                let floor_y: f32 = FOV as f32 / z * move_up_down;

                let mut rotate_x = floor_x * p.sin[p.angle as usize] - floor_y * p.cos[p.angle as usize] - (p.pos.y as f32 / 15.0);
                let mut rotate_y = floor_x * p.cos[p.angle as usize] + floor_y * p.sin[p.angle as usize] + (p.pos.x as f32 / 15.0);


                if rotate_x < 0.0 {rotate_x = -rotate_x + 1.0}
                if rotate_y < 0.0 {rotate_y = -rotate_y + 1.0}
                if rotate_x <= 0.0 || rotate_y <= 0.0 || rotate_x >= 5.0 || rotate_y >= 5.0 {continue;}     // Drawing a small square
             
                if rotate_x as i32 % 2 == rotate_y as i32 % 2 {pixel((x+ offset_x) as f32, (y+ offset_y) as f32,  255,0,0,  true, window);}
                else {pixel((x+ offset_x) as f32, (y+ offset_y) as f32,  0,255,0,  true, window);}
            }
        }
    }

    fn wall(&mut self, mut x1: i32, mut x2: i32,  b1: i32, b2: i32,  t1: i32, t2: i32,  s: usize,  w: i32, face: i32,  p: &Player, window: &mut RenderWindow) {
        let width  = RENDER_W as i32;
        let height = RENDER_H as i32;
        let mut verts: Vec<Vertex> = Vec::new();

        let wt = self.walls_data[w as usize].texture;
        let mut horizontal_texture: f32 = 0.0;
        let horizontal_step: f32 = ((self.textures[wt as usize].width * self.walls_data[w as usize].v) as f32) / (x2-x1) as f32;


        let delta_y_bottom = b2 - b1;
        let delta_y_top    = t2 - t1;

        let mut delta_x = x2-x1; if delta_x == 0 {delta_x = 1;}
        let starting_x = x1;

        // Don't draw behind camera
        if x1 < 1       {horizontal_texture = horizontal_texture - horizontal_step * x1 as f32; x1 = 1;}
        if x2 < 1       {x2 = 1;}
        if x1 > width-1 {x1 = width-1;}
        if x2 > width-1 {x2 = width-1;}
        
        for x in x1..x2 {
            let mut y1 = delta_y_bottom * (f32::floor((x - starting_x) as f32 + 0.5)) as i32 / delta_x + b1;
            let mut y2 = delta_y_top    * (f32::floor((x - starting_x) as f32 + 0.5)) as i32 / delta_x + t1;

            let mut vertical_texture: f32 = 0.0;
            let vertical_step: f32 = (self.textures[wt as usize].height as f32) / (y2-y1) as f32;

            // Clip Y axis - don't draw where camera doesn't see
            if y1 < 1        {vertical_texture = vertical_texture - vertical_step * y1 as f32; y1 = 1;}
            if y2 < 1        {y2 = 1;}
            if y1 > height-1 {y1 = height-1;}
            if y2 > height-1 {y2 = height-1;}


            // for sector in self.sectors_data.iter_mut() {
            //     if sector.surface == 1 {
            //         sector.surf_arr[x as usize] = y1;
            //     }
            //     if sector.surface == 2 {
            //         sector.surf_arr[x as usize] = y2;
            //     }

            //     if sector.surface == -1 {
            //         for y in (sector.surf_arr[x as usize]) .. y1 {
            //             verts.extend(Self::pixel(x as f32, y as f32, floors, false, window).unwrap());
            //         }
            //     }
            //     if sector.surface == -2 {
            //         for y in y2 .. (sector.surf_arr[x as usize]) {
            //             verts.extend(Self::pixel(x as f32, y as f32, ceiling, false, window).unwrap());
            //         }
            //     }
            // }

            // Walls
            if face == 0 {
                if self.sectors_data[s].surface == 1 {self.sectors_data[s].surf_arr[x as usize] = y1;}      // Bottom
                if self.sectors_data[s].surface == 2 {self.sectors_data[s].surf_arr[x as usize] = y2;}      // Top
                for y in y1..y2 {
                    // let p = (((self.textures[wt as usize].height - vertical_texture as i32 - 1)*3) * 
                    //            self.textures[wt as usize].width + (horizontal_texture as i32 * 3)) as usize;
                    // let p = (((self.textures[wt as usize].height - vertical_texture as i32 - 1)*3) * 
                    //            self.textures[wt as usize].width + ((horizontal_texture as i32 % self.textures[wt as usize].width) * 3)) as usize;
                    let p = (((self.textures[wt as usize].height - (vertical_texture as i32 % self.textures[wt as usize].height) - 1)*3) * 
                               self.textures[wt as usize].width + ((horizontal_texture as i32 % self.textures[wt as usize].width) * 3)) as usize;


                    let r: u8  = self.textures[wt as usize].data[p+0] - self.walls_data[w as usize].shade;
                    let g: u8  = self.textures[wt as usize].data[p+1] - self.walls_data[w as usize].shade;
                    let b: u8  = self.textures[wt as usize].data[p+2] - self.walls_data[w as usize].shade;

                    verts.extend(pixel(x as f32, y as f32,  r,g,b,  false, window).unwrap());
                    vertical_texture = vertical_texture + vertical_step;
                } horizontal_texture = horizontal_texture + horizontal_step;
            }

            // Top and bottom
            if face == 1 {
                // for y in y1..y2 {
                //     verts.extend(pixel(x as f32, y as f32,  255,0,0,  false, window).unwrap());    // Walls
                // }


                let offset_x = RENDER_W as i32 / 2;
                let offset_y = RENDER_H as i32 / 2;
                let persp_x  = x - offset_x;
                let mut wall_offset = 0;
                let tile = self.sectors_data[s].texture_scale * 7;

                if self.sectors_data[s].surface == 1 {y2 = self.sectors_data[s].surf_arr[x as usize]; wall_offset = self.sectors_data[s].z1;}      // Bottom
                if self.sectors_data[s].surface == 2 {y1 = self.sectors_data[s].surf_arr[x as usize]; wall_offset = self.sectors_data[s].z2;}      // Top
        
                let mut look_up_down = -p.look_up_down as f32 * 6.28;
                if look_up_down > RENDER_H {look_up_down = RENDER_H}
        
                let mut move_up_down = (p.pos.z as f32 - wall_offset as f32) / offset_y as f32;
                if move_up_down == 0.0 {move_up_down = 0.001;}
        
        
                let start_y = y1 - offset_y;
                let end_y = y2 - offset_y;
        
                for y in start_y as i32 .. end_y as i32 {
                    let mut z = y as f32 + look_up_down;
                    if z == 0.0 {z = 0.0001}
    
                    let floor_x: f32 = persp_x as f32 / z * move_up_down * tile as f32;
                    let floor_y: f32 =     FOV as f32 / z * move_up_down * tile as f32;
    
                    let mut rotate_x = floor_x * p.sin[p.angle as usize] - floor_y * p.cos[p.angle as usize] + (p.pos.y as f32 / 60.0 * tile as f32);
                    let mut rotate_y = floor_x * p.cos[p.angle as usize] + floor_y * p.sin[p.angle as usize] - (p.pos.x as f32 / 60.0 * tile as f32);
    
    
                    if rotate_x < 0.0 {rotate_x = -rotate_x + 1.0}
                    if rotate_y < 0.0 {rotate_y = -rotate_y + 1.0}
                    
                    // if rotate_x as i32 % 2 == rotate_y as i32 % 2 {pixel((persp_x + offset_x) as f32, (y + offset_y) as f32,  255,0,0,  true, window);}
                    // else {pixel((persp_x + offset_x) as f32, (y + offset_y) as f32,  0,255,0,  true, window);}
                    let st = self.sectors_data[s].surface_texture;

                    let p = (((self.textures[st as usize].height - (rotate_y as i32 % self.textures[st as usize].height) - 1)*3) * 
                               self.textures[st as usize].width + ((rotate_x as i32 % self.textures[st as usize].width) * 3)) as usize;

                    let r: u8  = self.textures[st as usize].data[p+0];
                    let g: u8  = self.textures[st as usize].data[p+1];
                    let b: u8  = self.textures[st as usize].data[p+2];

                    verts.extend(pixel((persp_x + offset_x) as f32, (y + offset_y) as f32,  r,g,b,  false, window).unwrap());
                }
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
        // Texture::test_textures(0, &self.textures, window);


        let width  = RENDER_W as i32;
        let height = RENDER_H as i32;

        let mut wx: [i32; 4] = [0; 4];
        let mut wy: [i32; 4] = [0; 4];
        let mut wz: [i32; 4] = [0; 4];

        let cos: f32 = p.cos[usize::try_from(p.angle).unwrap()];
        let sin: f32 = p.sin[usize::try_from(p.angle).unwrap()];

        let mut cycles: i32;


        for s in 0..(self.sectors_data.len()-1) as usize {
            for w in 0..(self.sectors_data.len()-s-1) {
                if self.sectors_data[w].dist < self.sectors_data[w+1].dist {
                    self.sectors_data.swap(w, w+1);
                }
            }
        }


        for s in 0..self.sectors_data.len() {
            self.sectors_data[s].dist = 0;   // Clear distance (drawing order)
            
            if p.pos.z < self.sectors_data[s].z1 {
                self.sectors_data[s].surface = 1; 
                cycles = 2;
                for x in 0..(RENDER_W as usize) {
                    self.sectors_data[s].surf_arr[x] = RENDER_H as i32;
                }

            } else if p.pos.z > self.sectors_data[s].z2 {
                self.sectors_data[s].surface = 2; 
                cycles = 2;
                for x in 0..(RENDER_W as usize) {
                    self.sectors_data[s].surf_arr[x] = 0;
                }

            } else {
                self.sectors_data[s].surface = 0; 
                cycles = 1;
            }

            for l in 0..cycles {
                for w in self.sectors_data[s].ws .. self.sectors_data[s].we {        
                    // Offset bottom 2 point by player
                    let mut x1: i32 = self.walls_data[w as usize].x1 - p.pos.x; 
                    let mut y1: i32 = self.walls_data[w as usize].y1 - p.pos.y;
                    
                    let mut x2: i32 = self.walls_data[w as usize].x2 - p.pos.x; 
                    let mut y2: i32 = self.walls_data[w as usize].y2 - p.pos.y;
                    
                    // Don't draw backfaces if we can't see them
                    if l == 1 {
                        swap(&mut x1, &mut x2);
                        swap(&mut y1, &mut y2);
                    }

    
                    // World X position
                    wx[0] = f32::floor((x1 as f32)*cos  -  (y1 as f32)*sin) as i32;
                    wx[1] = f32::floor((x2 as f32)*cos  -  (y2 as f32)*sin) as i32;
                    wx[2] = wx[0];
                    wx[3] = wx[1];
            
                    // World Y position
                    wy[0] = f32::floor((y1 as f32)*cos  +  (x1 as f32)*sin) as i32;
                    wy[1] = f32::floor((y2 as f32)*cos  +  (x2 as f32)*sin) as i32;   // Depth - how far wall is from the camera
                    wy[2] = wy[0];
                    wy[3] = wy[1];

                    // Walls distance - this is drawing order
                    self.sectors_data[s].dist = 
                        f32::floor((self.sectors_data[s].dist + Self::dist(0, 0,  (wx[0]+wx[1])/2,  (wy[0]+wy[1])/2)) as f32) as i32;
    
                    // World Z height
                    wz[0] = self.sectors_data[s].z1 - p.pos.z + ((p.look_up_down * wy[0])/32);
                    wz[1] = self.sectors_data[s].z1 - p.pos.z + ((p.look_up_down * wy[1])/32);
                    wz[2] = self.sectors_data[s].z2 - p.pos.z + ((p.look_up_down * wy[0])/32);
                    wz[3] = self.sectors_data[s].z2 - p.pos.z + ((p.look_up_down * wy[1])/32);
            
            
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
                    wx[0] = wx[0]*FOV / wy[0]+(width/2);  wy[0] = wz[0]*FOV / wy[0]+(height/2);
                    wx[1] = wx[1]*FOV / wy[1]+(width/2);  wy[1] = wz[1]*FOV / wy[1]+(height/2);
                    wx[2] = wx[2]*FOV / wy[2]+(width/2);  wy[2] = wz[2]*FOV / wy[2]+(height/2);
                    wx[3] = wx[3]*FOV / wy[3]+(width/2);  wy[3] = wz[3]*FOV / wy[3]+(height/2);
            
                    
                    let (x1, x2,  y1, y2,  y3, y4) = (wx[0], wx[1],  wy[0], wy[1],  wy[2], wy[3]);
                    self.wall(x1, x2,  y1, y2,  y3, y4,  s, w, l,  p, window);
                }
    
                self.sectors_data[s].dist = self.sectors_data[s].dist  /  (self.sectors_data[s].we - self.sectors_data[s].ws); 
            }
        }
    }
}