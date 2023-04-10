// I am almost sure, that it'll bite me in the ass for some cross refference
// pub mod Wall;
// pub mod Ray;

// class Player {..};  -  class in C++
// struct + impl       -  "class" in Rust   -  `trait` for polymorphism (to be checked)

// class Player : Entity {}        -  class inheritance in C++
// struct Player {parent: Entity}  -  class inheritance in Rust

use sfml::{
    graphics::{RectangleShape, Color},
    system::{Vector2f},
};

pub struct Player {
    pub fov: f32,
    pub rotation: f32,

    pub player: RectangleShape<'static>,
    pub color: Color,
    
    pub size: Vector2f,
    pub position: Vector2f,
}
impl Player {
    // Constructor
    pub fn new(p: Vector2f, s: Vector2f, f: f32) -> Player {
        Player {
            player: RectangleShape::new(),
            
            fov: f,
            rotation: 0.0,

            size: s, 
            position: p,

            color: Color::RED,
        }
    }

    pub fn set_color(self) -> () {
        return;
    }


    // pub fn set_color(&mut self, c: Color) -> () {
    //     self.color = c;                 // We don't need to explicitly say `return` in Rust       
    //     // return;                      // So we need to say `return;` here, to not return `Player`
    //                                     // or add a semicolon to the last line

    //     // Player{color: c, ..self};    // Some tests that didn't go well...
    // }
    // pub fn get_color(self) -> Color {
    //     self.color                      // Here we want to return something, no semicolon
    //     // return self.color;           // It does the same thing
    // }


    // pub fn color(mut self, c: Color) -> Player {self.color = c; self}
    // pub fn size(mut self, s: Vector2f) -> Player {self.size = s; self}
    // pub fn position(mut self, p: Vector2f) -> Player {self.position = p; self}


}