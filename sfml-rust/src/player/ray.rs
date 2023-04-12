use sfml::graphics::{Color};
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

        let mut dof: i32;             // Deepth of field
        let fov = f;             // Field of view

        let mut ray_x = 0.0;     // Ray X position
        let mut ray_y = 0.0;     // Ray Y position
        let mut ray_ang: f32;         // Ray angle

        let cell = map.cell;     // Cell size
        let map_w = map.width;   // Map width
        let map_h = map.height;  // Map height
        let mut map_tv = 0;      // Cell type on vertical axis (0, 1, 2, 3 etc.)
        let mut map_th = 0;      // Cell type on horizontal axis (0, 1, 2, 3 etc.)
        
        let mut map_x: i32;           // On what cell, on X axis, player is standing
        let mut map_y: i32;           // On what cell, on Y asix, player is standing
        let mut map_pos: i32;         // Index for map array

        let mut offset_x = 0.0;      // Ray offset on X axis
        let mut offset_y = 0.0;      // Ray offset on Y axis
        let grid = &map.grid;  // Our map

        let player_x = player.player.position().x;  // Player position on X axis in general
        let player_y = player.player.position().y;  // Player position on Y axis in general
        let player_ang = radians(player.player.rotation());  // Player angle (heading)

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

        for r in 0..fov as i32 {
            // Horizontal line
            let mut dist_h = f32::INFINITY;     // We want some big number here for the start
            let mut hor_x = player_x;
            let mut hor_y = player_y;

            dof = 0;
            let a_tan = -1.0/f32::tan(ray_ang);


            if ray_ang > PI {
                ray_y = (((player_y as i32)/cell)*cell) as f32 - 0.0001;   // Rounding to the very edge of the wall
                ray_x = (player_y - ray_y) * a_tan + player_x;

                offset_y = -cell as f32;
                offset_x = -offset_y * a_tan;
            }
            if ray_ang < PI {
                ray_y = ((((player_y as i32)/cell)*cell) + cell) as f32;
                ray_x = (player_y - ray_y) * a_tan + player_x;

                offset_y = cell as f32;
                offset_x = -offset_y * a_tan;
            }
            if ray_ang == 0.0 || ray_ang == PI {
                // Lines here are parallel - they never meet 
                ray_x = player_x;
                ray_y = player_y;
                dof = 8;
            }


            while dof < 8 {
                map_x = (ray_x as i32)/cell;
                map_y = (ray_y as i32)/cell;
                map_pos = map_y * map_w + map_x;    // Index for map grid - that's why it is 1D array

                if (map_pos > 0) && (map_pos < map_w * map_h) && (grid[map_pos as usize] > 0) {
                    // We found the shortest path to horizontal wall
                    map_th = grid[map_pos as usize];    // Map has different kinds of cells
                    hor_x = ray_x;
                    hor_y = ray_y;
                    dist_h = Self::dist(player_x, player_y,  hor_x, hor_y);

                    dof = 8;
                } else {
                    // We're still searching
                    ray_x = ray_x + offset_x;
                    ray_y = ray_y + offset_y;
                    dof = dof + 1;
                }
            }


            // Vertical line
            let mut dist_v = f32::INFINITY;     // We want some big number here for the start
            let mut vert_x = player_x;
            let mut vert_y = player_y;

            dof = 0;
            let n_tan = -f32::tan(ray_ang);

            
            if ray_ang > FRAC_PI_2 && ray_ang < 3.0*FRAC_PI_2 {
                ray_x = (((player_x as i32)/cell)*cell) as f32 - 0.0001;   // Rounding to the very edge of the wall
                ray_y = (player_x - ray_x) * n_tan + player_y;

                offset_x = -cell as f32;
                offset_y = -offset_x * n_tan;
            }
            if ray_ang < FRAC_PI_2 || ray_ang > 3.0*FRAC_PI_2 {
                ray_x = ((((player_x as i32)/cell)*cell) + cell) as f32;
                ray_y = (player_x - ray_x) * n_tan + player_y;

                offset_x = cell as f32;
                offset_y = -offset_x * n_tan;
            }
            if ray_ang == 0.0 || ray_ang == PI {
                // Lines here are parallel - they never meet 
                ray_x = player_x;
                ray_y = player_y;
                dof = 8;
            }


            while dof < 8 {
                map_x = (ray_x as i32)/cell;
                map_y = (ray_y as i32)/cell;
                map_pos = map_y * map_w + map_x;    // Index for map grid - that's why it is 1D array

                if (map_pos > 0) && (map_pos < map_w * map_h) && (grid[map_pos as usize] > 0) {
                    // We found the shortest path to horizontal wall
                    map_tv = grid[map_pos as usize];    // Map has different kinds of cells
                    vert_x = ray_x;
                    vert_y = ray_y;
                    dist_v = Self::dist(player_x, player_y,  vert_x, vert_y);

                    dof = 8;
                } else {
                    // We're still searching
                    ray_x = ray_x + offset_x;
                    ray_y = ray_y + offset_y;
                    dof = dof + 1;
                }
            }


            // Distance based shading for 3D walls and colouring based on cell type
            let mut wallpaint = Color::rgb(0, 0, 0);
            let brightness: f32;
            let width_sqr = (512*512) as f32;     // Magic number - screen is 1024, but we're drawing only on the half of it 
            let dist_sqr: f32;

            // We only want to draw shortes ray from dist_v and dist_h
            if dist_v < dist_h {
                ray_x = vert_x;
                ray_y = vert_y;
                dist = dist_v;


                dist_sqr = dist*dist;
                brightness = Self::map(dist_sqr, 0.0, width_sqr, 255.0, 0.0);

                // Shading and slightly different from horizontal walls color
                match map_tv {
                    1 => {wallpaint.r = brightness as u8;},
                    2 => {wallpaint.g = brightness as u8;},
                    3 => {wallpaint.b = brightness as u8;},
                    4 => {wallpaint.r = brightness as u8; wallpaint.g = brightness as u8;},
                    _ => {wallpaint = Color::BLACK;},
                }
            } else if dist_h < dist_v {
                ray_x = hor_x;
                ray_y = hor_y;
                dist = dist_h;

                dist_sqr = dist*dist;
                brightness = Self::map(dist_sqr, 0.0, width_sqr, 230.0, 0.0);
            
                match map_th {
                    1 => {wallpaint.r = brightness as u8;},
                    2 => {wallpaint.g = brightness as u8;},
                    3 => {wallpaint.b = brightness as u8;},
                    4 => {wallpaint.r = brightness as u8; wallpaint.g = brightness as u8;},
                    _ => {wallpaint = Color::BLACK;},
                }
            } else {brightness = 127.0; wallpaint.b = brightness as u8;}

            let ray = WideLine::new(Vector2f::new(player_x, player_y), Vector2f::new(ray_x, ray_y), 1.0, wallpaint);
            ray.draw(window);
            
            ray_ang = ray_ang + radians(1.0);   // Magic number - offset each ray by 1 degree
            if ray_ang < 0.0    {ray_ang = ray_ang + 2.0*PI;}
            if ray_ang > 2.0*PI {ray_ang = ray_ang - 2.0*PI;}


            // 3D walls
            // Removes fish eye effect
            let mut cell_ang = player_ang - ray_ang;
            if cell_ang < 0.0    {cell_ang = cell_ang + 2.0*PI;}
            if cell_ang > 2.0*PI {cell_ang = cell_ang - 2.0*PI;}
            dist = dist * f32::cos(cell_ang);


            let line_h = (cell*400) as f32 / dist;   // Walls height - can be regulated

            // // Without this guard u8 will overflow       -  this is true for C++, in Rust I've got no artifacts
            // if brightness < 0.0   {brightness = 0.0;}
            // if brightness > 255.0 {brightness = 255.0;}

            let wall_width = 512/cell;              // Magic number - 512 is viewport size  -  space taken on the screen
            let line_offset = 240.0 - line_h/2.0;   // Magic number - 240 is camera height
        
            let wall = WideLine::new(Vector2f::new((r*wall_width) as f32 + 530.0, line_offset), Vector2f::new((r*wall_width) as f32 + 530.0, line_h+line_offset), wall_width as f32, wallpaint);
            wall.draw(window);
        }


    }

    // Distance between two points
    fn dist(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
        f32::sqrt(f32::powi(ax-bx, 2) + f32::powi(ay-by, 2))
    }

    // fn map<T>(num: T, inMin: T, inMax: T, outMin: T, outMax: T) -> T {
    //     (num-inMin) * (outMax-outMin)/(inMax-inMin) + outMin
    // }

    // Map one range to another
    fn map(num: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
        (num-in_min) * (out_max-out_min)/(in_max-in_min) + out_min
    }
}