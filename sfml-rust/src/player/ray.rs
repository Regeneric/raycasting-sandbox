use sfml::graphics::{Color, RenderTarget, Texture, IntRect, RcTexture, RenderStates, Text, Font, Sprite, RectangleShape, Shape};
use sfml::graphics::{RenderWindow, Transformable};
use sfml::system::Vector2f;

use std::f32::consts::PI;           // For PI   constant (f32)
use std::f32::consts::FRAC_PI_2;    // For PI/2 constant (f32)

use crate::radians;
use super::wline::WideLine;
use super::{Player, Wall};  // We want to use structs from from modules used in parent module

// use cpp:cpp;

pub struct Ray;     // Ray stores nothing
impl Ray {
    pub fn cast(f: f32, player: &Player, map: &Wall, window: &mut RenderWindow) {
        let mut dist = 0.0;      // Shortest distance to wall

        let width  = window.size().x;
        let height = window.size().y;

        let dof_range = 64;
        let mut dof: i32;               // Deepth of field
        let fov = f;                    // Field of view

        let mut ray_x = 0.0;            // Ray X position
        let mut ray_y = 0.0;            // Ray Y position
        let mut ray_ang: f32;           // Ray angle

        let cell = map.cell as i64;     // Cell size
        let map_w = map.width as i64;   // Map width
        let map_h = map.height as i64;  // Map height
        let mut map_tv = 0;             // Cell type on vertical axis (0, 1, 2, 3 etc.)
        let mut map_th = 0;             // Cell type on horizontal axis (0, 1, 2, 3 etc.)
        
        let mut map_x: i64;             // On what cell, on X axis, player is standing
        let mut map_y: i64;             // On what cell, on Y asix, player is standing
        let mut map_pos: i64;           // Index for map array

        let mut offset_x = 0.0;         // Ray offset on X axis
        let mut offset_y = 0.0;         // Ray offset on Y axis
        let grid = &map.grid;           // Our map

        let player_x = player.player.position().x;              // Player position on X axis in general
        let player_y = player.player.position().y;              // Player position on Y axis in general
        let player_ang = radians(player.player.rotation());     // Player angle (heading)



        // Difference between (map_x, map_y) and (player_x, player_y) is simple:
        // (0, 1) vs (436.11315, 313.21948)
        // (5, 6) vs (101.31454, 201.41210)
        // (map_x, map_y) are coordinates for map from `map.grid`
        // (player_x, player_y) are coordinates for position in the window

        // `ray_ang`  vs  `player_ang` is also simple
        // `player_ang` is heading - based on that we know if we're moving right, top etc.
        // `ray_ang` is an angle between -(fov/2) and (fov/2) - we're drawing rays between this range


        // Ray angle is in radians
        ray_ang = player_ang - radians(fov/2.0);
        if ray_ang < 0.0    {ray_ang = ray_ang + 2.0*PI;}
        if ray_ang > 2.0*PI {ray_ang = ray_ang - 2.0*PI;}

        for r in 0..width as i32 {
            // Horizontal line
            let mut dist_h = f32::INFINITY;     // We want some big number here for the start
            // let mut hor_x = player_x;
            // let mut hor_y = player_y;

            dof = 0;
            let a_tan = -1.0/f32::tan(ray_ang);


            if ray_ang > PI {
                ray_y = (((player_y as i64)/cell)*cell) as f32 - 0.0001;   // Rounding to the very edge of the wall
                offset_y = -cell as f32;
            }
            if ray_ang < PI {
                ray_y = ((((player_y as i64)/cell)*cell) + cell) as f32;
                offset_y = cell as f32;
            }
            if ray_ang == 0.0 || ray_ang == PI {
                // Lines here are parallel - they never meet 
                ray_x = player_x;
                ray_y = player_y;
                dof = dof_range;
            }

            ray_x = (player_y - ray_y) * a_tan + player_x;
            offset_x = -offset_y * a_tan;


            while dof < dof_range {
                map_x = (ray_x as i64)/cell;
                map_y = (ray_y as i64)/cell;
                map_pos = map_y * map_w + map_x;        // Index for map grid - that's why it is 1D array

                if (map_pos > 0) && (map_pos < map_w * map_h) && (grid[map_pos as usize] > 0) {
                    // We found the shortest path to horizontal wall
                    map_th = grid[map_pos as usize];    // Map has different kinds of cells
                    // hor_x = ray_x;
                    // hor_y = ray_y;
                    dist_h = Self::dist(player_x, player_y,  ray_x, ray_y);

                    dof = dof_range;
                } else {
                    // We're still searching
                    ray_x = ray_x + offset_x;
                    ray_y = ray_y + offset_y;
                    dof = dof + 1;
                }
            }


            // sprite.set_scale(Vector2f::new(texture_offset_x_hor as f32, texture_offset_y_hor as f32));
            // sprite.set_position(Vector2f::new(texture_offset_x_hor as f32, texture_offset_y_hor as f32));


            // Vertical line
            let mut dist_v = f32::INFINITY;     // We want some big number here for the start
            let mut vert_x = player_x;
            let mut vert_y = player_y;

            dof = 0;
            let n_tan = -f32::tan(ray_ang);

            
            if ray_ang > FRAC_PI_2 && ray_ang < 3.0*FRAC_PI_2 {
                ray_x = (((player_x as i64)/cell)*cell) as f32 - 0.0001;   // Rounding to the very edge of the wall
                offset_x = -cell as f32;
            }
            if ray_ang < FRAC_PI_2 || ray_ang > 3.0*FRAC_PI_2 {
                ray_x = ((((player_x as i64)/cell)*cell) + cell) as f32;
                offset_x = cell as f32;
                
            }
            if ray_ang == 0.0 || ray_ang == PI {
                // Lines here are parallel - they never meet 
                ray_x = player_x;
                ray_y = player_y;
                dof = dof_range;
            }

            ray_y = (player_x - ray_x) * n_tan + player_y;
            offset_y = -offset_x * n_tan;


            while dof < dof_range {
                map_x = (ray_x as i64)/cell;
                map_y = (ray_y as i64)/cell;
                map_pos = map_y * map_w + map_x;        // Index for map grid - that's why it is 1D array

                if (map_pos > 0) && (map_pos < map_w * map_h) && (grid[map_pos as usize] > 0) {
                    // We found the shortest path to horizontal wall
                    map_tv = grid[map_pos as usize];    // Map has different kinds of cells
                    // vert_x = ray_x;
                    // vert_y = ray_y;
                    dist_v = Self::dist(player_x, player_y,  ray_x, ray_y);

                    dof = dof_range;
                } else {
                    // We're still searching
                    ray_x = ray_x + offset_x;
                    ray_y = ray_y + offset_y;
                    dof = dof + 1;
                }
            }
            

            // sprite.set_scale(Vector2f::new(texture_offset_x_vert as f32, texture_offset_y_vert as f32));
            // sprite.set_position(Vector2f::new(texture_offset_x_vert as f32, texture_offset_y_vert as f32));


            // Distance based shading for 3D walls and colouring based on cell type
            let mut wallpaint = Color::rgb(0, 0, 0);
            let brightness: f32;
            let width_sqr = (width+width+width) as f32;    // Magic number - screen is 1024, but we're drawing only on the half of it 
            let dist_sqr: f32;

            // We only want to draw shortes ray from dist_v and dist_h
            if dist_v < dist_h {
                // ray_x = vert_x;
                // ray_y = vert_y;
                dist = dist_v;

                dist_sqr = dist*dist;
                brightness = Self::map(dist_sqr, 0.0, width_sqr, 255.0, 0.0);

                // Shading and slightly different from horizontal walls color
                match map_tv {
                    1 => {
                        wallpaint.r = (brightness/2.0) as u8;
                        wallpaint.g = (brightness/2.0) as u8;
                        wallpaint.b = (brightness/2.0) as u8;
                    },
                    2 => {wallpaint.r = brightness as u8;},
                    3 => {wallpaint.b = brightness as u8;},
                    4 => {wallpaint.r = brightness as u8; wallpaint.g = brightness as u8;},
                    _ => {wallpaint = Color::BLACK},
                }
            } else if dist_h < dist_v {
                // ray_x = hor_x;
                // ray_y = hor_y;
                dist = dist_h;

                dist_sqr = dist*dist;
                brightness = Self::map(dist_sqr, 0.0, width_sqr, 220.0, 0.0);
            
                match map_th {
                    1 => {
                        wallpaint.r = (brightness/2.0) as u8;
                        wallpaint.g = (brightness/2.0) as u8;
                        wallpaint.b = (brightness/2.0) as u8;
                    },
                    2 => {wallpaint.r = brightness as u8;},
                    3 => {wallpaint.b = brightness as u8;},
                    4 => {wallpaint.r = brightness as u8; wallpaint.g = brightness as u8;},
                    _ => {wallpaint = Color::BLACK;},
                }
            } else {brightness = 127.0; wallpaint.b = brightness as u8;}


            // let ray = WideLine::new(Vector2f::new(player_x, player_y), Vector2f::new(ray_x, ray_y), 1.0, wallpaint);
            // ray.draw(window);
            
            ray_ang = ray_ang + radians(fov / width as f32);   // Magic number - offset each ray by 1 degree

            if ray_ang < 0.0    {ray_ang = ray_ang + 2.0*PI;}
            if ray_ang > 2.0*PI {ray_ang = ray_ang - 2.0*PI;}


            // 3D walls
            // Removes fish eye effect
            let mut cell_ang = player_ang - ray_ang;
            if cell_ang < 0.0    {cell_ang = cell_ang + 2.0*PI;}
            if cell_ang > 2.0*PI {cell_ang = cell_ang - 2.0*PI;}
            dist = dist * f32::cos(cell_ang);


            let line_height = (cell*height as i64) as f32 / dist;   // Walls height - can be regulated
            let wall_width = 1;                                     // Space taken on the screen by single strip
            let line_offset = (height/2) as f32 - line_height/2.0;  // Camera height
            let wall_ofsset = 1.0;                                  // If we want to draw map and walls on the same time

            let wall = WideLine::new(Vector2f::new(((r as i32) * wall_width) as f32 + wall_ofsset, line_offset),                // FROM where
                                     Vector2f::new(((r as i32) * wall_width) as f32 + wall_ofsset, line_height+line_offset),    // TO   where
                                     wall_width as f32, wallpaint);
            wall.draw(window);


            // sprite.set_texture_rect(IntRect::new((r * wall_width) + (wall_ofsset as i32), line_offset as i32,  1,  (line_height + line_offset) as i32));
            // sprite.set_texture_rect(IntRect::new(0, 0,  1, 64));
            // sprite.set_texture_rect(IntRect::new(0, 0,  1, (line_height) as i32));
            // sprite.set_scale(Vector2f::new(1.0, line_height));
            
            // sprite.set_position(Vector2f::new(513.0, 0.0));
            
            
            // sprite.set_position(Vector2f::new(((r * wall_width) + (wall_ofsset as i32)) as f32, line_offset));
            // sprite.set_scale(Vector2f::new(1.0/cell as f32, line_height/cell as f32));
            // sprite.set_scale(Vector2f::new(1.0/cell as f32, line_height/cell as f32));

            // window.draw(&sprite);
        }
    }

    // Distance between two points
    fn dist(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
        f32::sqrt(f32::powi(ax-bx, 2) + f32::powi(ay-by, 2))
    }

    // Map one range to another
    fn map(num: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
        (num-in_min) * (out_max-out_min)/(in_max-in_min) + out_min
    }
}