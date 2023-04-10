// #include "Player.hpp"  -  include project files in C++
// mod Player;            -  include project files (crates) in Rust

// using namespace hkk;        -  use namespace defined in files (eg. hkk::Player -> Player) in C++
// use crate::Player::Player;  -  Rust creates namespace for all files, thus  crate::FileName::Struct; 

/*
Methods visible and available for other files are under  `public:`  in C++

class Player {
public:
    Player();
private:
    ..
};


In Rust we can use `pub` keyword

impl Player {
    pub fn player(&self) -> ();
}
 */

// pub mod Player;          // or we can use `pub` while including modules
mod player;                 // lowercase to avoid doing  `use crate::Player::Player as OtherPlayer;`
use crate::player::Player;

use sfml::{
    // audio::{Sound, SoundBuffer, SoundSource},
    system::{Vector2f},
    window::{ContextSettings, Event, Key, Style},
    graphics::{Color, RenderTarget, RenderWindow},
};

fn main() {
    // Define some constants
    let width = 800;
    let height = 600;

    // Create the window of the application
    let context_settings = ContextSettings {..Default::default()};
    let mut window = RenderWindow::new(
        (width, height),
        "SFML Rust",
        Style::CLOSE,
        &context_settings,
    );

    // let mut some_var = 10;   // If not used anywhere, Rust throws error
    // let mut _some_var = 10;  // Rust will ignore that this variable is unused

    let player = Player::new(Vector2f::new((width/2) as f32, (height/2) as f32), Vector2f::new(10.0, 10.0), 60.0);
    player.set_color();

    // player = player.set_color(Color::YELLOW);
    // player = player.set_color(Color::GREEN);
    // player = player.get_color();


    // loop - infinite loop untils  break;  or program exit  -  while(window.isOpen()) in C++ to catch that 
    loop {
        while let Some(event) = window.poll_event() {
            // swtich(event) {} in C++
            match event {
                Event::Closed => return,

                Event::KeyPressed {code: Key::Up, ..}    => {/*TODO move player up*/},
                Event::KeyPressed {code: Key::Down, ..}  => {/*TODO move player down*/},
                Event::KeyPressed {code: Key::Right, ..} => {/*TODO rotate player right*/},
                Event::KeyPressed {code: Key::Left, ..}  => {/*TODO rotate player left*/},

                _ => {}     // `default:` case in C++ `switch`
            }
        } window.clear(Color::rgb(80, 80, 80));

        // TODO
        // Draw map
        // Draw player
        // Cast rays
        // Draw 3D view

        window.display();
    }
}