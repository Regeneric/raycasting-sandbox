use std::f32::consts::{PI, FRAC_PI_2};

use sfml::{
    system::{Vector2f},
    window::{ContextSettings, Event, Key, Style},
    graphics::{Color, RenderTarget, RenderWindow, RectangleShape, Shape, Transformable},
};
use sfml_rust_fast::wline::WideLine;


const PLAYER_SIZE: f32 = 8.0; 
const CELL_SIZE: f32 = 64.0; 
const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const NAME: &str = "SFML Rust";


fn radians(angle: f32) -> f32 {angle * (PI/180.0)}

fn main() {
    let context_settings = ContextSettings {..Default::default()};
    let mut window = RenderWindow::new(
        (WIDTH, HEIGHT),
        NAME,
        Style::CLOSE,
        &context_settings,
    );


    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed {code: Key::Escape, ..} => return,
                _ => {}
            }
        } window.clear(Color::rgb(80, 80, 80));

        let mut map: [[i32; 8]; 8] = [
            [1,1,1,1,1,1,1,1],
            [1,0,1,0,0,0,0,1],
            [1,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,1],
            [1,0,0,1,0,0,0,1],
            [1,0,0,0,0,0,0,1],
            [1,0,0,0,0,1,0,1],
            [1,1,1,1,1,1,1,1]];

        // Iter over every element
        for (y, row) in map.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let mut wall = RectangleShape::new();

                if cell > &mut 0 {wall.set_fill_color(Color::BLACK);}
                else {wall.set_fill_color(Color::WHITE);}
                
                let xs = (x as f32) * CELL_SIZE;
                let ys = (y as f32) * CELL_SIZE;

                wall.set_size(Vector2f::new(CELL_SIZE-1.0, CELL_SIZE-1.0));
                wall.set_position(Vector2f::new(xs, ys));

                window.draw(&wall);
            }
        }

        let mut player = RectangleShape::new();
            player.set_size(Vector2f::new(PLAYER_SIZE, PLAYER_SIZE));
            player.set_origin(Vector2f::new(PLAYER_SIZE, PLAYER_SIZE));
            player.set_position(Vector2f::new((WIDTH as f32)/2.0, (HEIGHT as f32)/2.0));
            player.set_fill_color(Color::RED);
            player.set_rotation(45.0);
            // player.rotate(1.0);
        window.draw(&player);



        let player_rotation = player.rotation();
        let mut player_dir: Vector2f = Default::default();
            player_dir.x = f32::cos(player_rotation);
            player_dir.y = f32::sin(player_rotation);

        
        let player_x = player.position().x;
        let player_y = player.position().y; 




        // First intersection
        let mut ray_y: f32 = 0.0;
        let mut ray_x: f32 = 0.0;

        //    If the ray is facing up (> PI) 
        //      A.y = rounded_down(Py/64) * (64) - 1;

        //    If the ray is facing down (< PI)
        //      A.y = rounded_down(Py/64) * (64) + 64;
        
        // Map Y coordinate
        if radians(player_rotation) > PI {
            ray_y = (f32::floor(player_y/CELL_SIZE) * CELL_SIZE) - 1.0;
        }
        if radians(player_rotation) < PI {
            ray_y = (f32::floor(player_y/CELL_SIZE) * CELL_SIZE) + CELL_SIZE;
        } 
        if radians(player_rotation) == 0.0 || radians(player_rotation) == PI {
            ray_y = player_y;
            ray_x = player_x;
        }

        // Px + (Py-A.y)/tan(ALPHA)
        ray_x = player_x + (player_y - ray_y) / f32::tan(player_rotation);  // Map X cooridante


        let mut hor_x = 0.0;
        let mut hor_y = 0.0;

        let mut ver_x = 0.0;
        let mut ver_y = 0.0;

        
        let mut offset_x = 0.0;
        let mut offset_y = 0.0;

        let mut hit = 0;
        while hit == 0 {
            let map_x = f32::floor(ray_x/CELL_SIZE);
            let map_y = f32::floor(ray_y/CELL_SIZE);

            // println!("MX: {} ; MY: {}", map_x, map_y);
            // println!("MAP[MX][MY]: {}", map[map_x as usize][map_y as usize]);
            // println!("MAP[0][0]: {}", map[0 as usize][0 as usize]);
            // println!("MAP[7][7]: {}", map[7 as usize][7 as usize]);
            // println!("MAP[3][5]: {}", map[3 as usize][5 as usize]);

            if map[map_x as usize][map_y as usize] > 0 {hor_x = ray_x; hor_y = ray_y; break;}
            
            if radians(player_rotation) > PI {offset_y = -CELL_SIZE; offset_x = CELL_SIZE/f32::tan(player_rotation);}
            if radians(player_rotation) < PI {offset_y = CELL_SIZE;  offset_x = -CELL_SIZE/f32::tan(player_rotation);}

            ray_x = ray_x + offset_x;
            ray_y = ray_y + offset_y;
        }
        


        // Map Y coordinate
        if radians(player_rotation) > FRAC_PI_2 && radians(player_rotation) < 3.0*FRAC_PI_2 {
            ray_x = (f32::floor(player_x/CELL_SIZE) * CELL_SIZE) + CELL_SIZE;
        }
        if radians(player_rotation) < FRAC_PI_2 || radians(player_rotation) > 3.0*FRAC_PI_2 {
            ray_x = (f32::floor(player_x/CELL_SIZE) * CELL_SIZE) - 1.0; 
        } 
        if radians(player_rotation) == 0.0 || radians(player_rotation) == PI {
            ray_y = player_y;
            ray_x = player_x;
        }

        // Px + (Py-A.y)/tan(ALPHA)
        ray_y = player_y + (player_x - ray_x) * f32::tan(player_rotation);  // Map X cooridante


        while hit == 0 {
            let map_x = f32::floor(ray_x/CELL_SIZE);
            let map_y = f32::floor(ray_y/CELL_SIZE);

            // println!("MX: {} ; MY: {}", map_x, map_y);
            // println!("MAP[MX][MY]: {}", map[map_x as usize][map_y as usize]);
            // println!("MAP[0][0]: {}", map[0 as usize][0 as usize]);
            // println!("MAP[7][7]: {}", map[7 as usize][7 as usize]);
            // println!("MAP[3][5]: {}", map[3 as usize][5 as usize]);

            if map[map_x as usize][map_y as usize] > 0 {ver_x = ray_x; ver_y = ray_y; break;}
            
            if radians(player_rotation) > FRAC_PI_2 && radians(player_rotation) < 3.0*FRAC_PI_2 {offset_x = -CELL_SIZE; offset_y = CELL_SIZE/f32::tan(player_rotation);}
            if radians(player_rotation) < FRAC_PI_2 || radians(player_rotation) > 3.0*FRAC_PI_2 {offset_x = CELL_SIZE;  offset_y = -CELL_SIZE/f32::tan(player_rotation);}

            ray_x = ray_x + offset_x;
            ray_y = ray_y + offset_y;
        }

        let distH = f32::sqrt((player_x-hor_x)*(player_x-hor_x) + (player_y-hor_y)*(player_y-hor_y));
        let distV = f32::sqrt((player_x-ver_x)*(player_x-ver_x) + (player_y-ver_y)*(player_y-ver_y));

        if distV < distH {ray_x = ver_x; ray_y = ver_y;}
        if distH < distV {ray_x = hor_x; ray_y = hor_y;}

        let ray = WideLine::new(Vector2f::new(player_x, player_y), Vector2f::new(ray_x, ray_y), 1.0, Color::RED);
        ray.draw(&mut window);

        window.display();
    }
}