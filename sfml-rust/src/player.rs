// class Player {..};  -  class in C++
// struct + impl       -  "class" in Rust

// class Player : Entity {}        -  class inheritance in C++
// struct Player {parent: Entity}  -  class inheritance in Rust


pub mod wall;                       // Including submodule `wall` as public, so other modules that include `player` can use it too
pub use crate::player::wall::Wall;  // Using this crate scope as public for other modules that include `player`

pub mod ray;                        // Same goes for ray
pub use crate::player::ray::Ray;

mod wline;   

use sfml::{
    graphics::{RectangleShape, Color, Transformable, RenderWindow, RenderTarget, Shape},
    system::{Vector2f},
};

pub struct Player<'a> {
    pub fov: f32,
    pub rotation: f32,

    pub player: RectangleShape<'a>,
    pub color: Color,
    
    pub size: Vector2f,
    pub position: Vector2f,
}

impl<'a> Player<'a> {
    // Constructor
    pub fn new(p: Vector2f, s: Vector2f, f: f32) -> Self {
        // Setting variables and stuff before assigning them to struct
        let c = Color::RED;
        let mut r = RectangleShape::new();
            r.set_size(s);
            r.set_position(p);
            r.set_fill_color(c);
            r.set_origin(Vector2f::new(s.x/2.0, s.y/2.0));


        // We return struct `Player` from the constructor
        Player {
            player: r,
            
            fov: f,
            rotation: 0.0,

            size: s, 
            position: p,

            color: c,
        }
    }

    pub fn advance(&mut self, p: Vector2f, map: &Wall) -> () {
        self.position = p;
        let grid = &map.grid;


        // Collision detection with wall sliding
        let dist_from_wall = 12.0;
        let mut player_x = self.player.position().x;
        let mut player_y = self.player.position().y;


        let mut offset_x = 0;
        if self.position.x < 0.0 {offset_x = -1 * dist_from_wall as i32;}
        else {offset_x = dist_from_wall as i32;}

        let mut offset_y = 0;
        if self.position.y < 0.0 {offset_y = -1 * dist_from_wall as i32;}
        else {offset_y = dist_from_wall as i32;}

        let grid_pos_x = player_x as i32 / map.cell;
        let grid_pos_off_x_add = (player_x as i32 + offset_x) / map.cell;

        let grid_pos_y = player_y as i32 / map.cell;
        let grid_pos_off_y_add = (player_y as i32 + offset_y) / map.cell;
    

        if grid[(grid_pos_y * map.width  +  grid_pos_off_x_add) as usize] == 0 {
            player_x = player_x + self.position.x;
            self.player.set_position(Vector2f::new(player_x, player_y));
        }
        if grid[(grid_pos_off_y_add * map.width  +  grid_pos_x) as usize] == 0 {
            player_y = player_y + self.position.y;
            self.player.set_position(Vector2f::new(player_x, player_y));
        }
    }

    pub fn rotate(&mut self, a: f32) -> () {
        // self.rotation = self.rotation + a;
        self.player.rotate(a);
        self.rotation = self.player.rotation();
        // self.rotation = self.rotation + a;
    }


    pub fn draw(&self, window: &mut RenderWindow) -> () {
        window.draw(&self.player);

        // In C++ I would do something like this:
        //
        // void draw(RenderWindow *window) {
        //     window->draw(player);
        // }
        //
        // In C++ `window` is a pointer, so in `main()` i would pass
        // a reference as a function argument, like: `player.draw(&window);`
        // Calling it in Rust goes like this: `player.draw(&mut window);`
        // 
        // If I understand it correctly, Rust wants me to use reference (&), 
        // so I will just borrow, not move ownership, and then I need to
        // make that reference mutable (mut), so I can do stuff with it.

    }

    pub fn look(&mut self, map: &Wall, window: &mut RenderWindow) -> () {
        Ray::cast(self.fov, &self, &map, window);
    }
}