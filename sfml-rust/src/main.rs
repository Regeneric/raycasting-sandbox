// #include "Player.hpp"  -  include project files in C++
// mod player;            -  include project files (crates) in Rust

// using namespace hkk;        -  use namespace defined in files (eg. hkk::Player -> Player) in C++
// use crate::player::Player;  -  Rust creates namespace for all files, thus  crate::FileName::Struct; 

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

mod player;                     // This is main.rs, so we don't need to `pub mod` this module
use crate::player::Player;      // Lowercase to avoid doing  `use crate::Player::Player as OtherPlayer;
    use crate::player::Wall;    // Submodule of module player (player/wall.rs)
//    use crate::player::Ray;   // Submodule of module player (player/ray.rs)

use std::f32::consts::PI;       // For PI constant (f32)

use sfml::graphics::View;
use sfml::{
    // audio::{Sound, SoundBuffer, SoundSource},
    system::{Vector2f, Clock},
    window::{ContextSettings, Event, Key, Style},
    graphics::{Color, RenderTarget, RenderWindow},
};


// Heading vector from given angle
fn from_angle(a: f32) -> Vector2f {Vector2f::new(f32::cos(a)/25.0, f32::sin(a)/25.0)}

// Degrees to radians
fn radians(a: f32) -> f32 {a * (PI/180.0)}


fn main() {
    // `viewport_` slice this window in half, so we can draw two images
    let width = 1024;
    let height = 720;
    let viewport_width = width/2;

    // Create the window of the application
    // It's SFML standard, so pretty much the same as in C++
    let context_settings = ContextSettings {..Default::default()};
    let mut window = RenderWindow::new(
        (width, height),
        "SFML Rust",
        Style::CLOSE,
        &context_settings,
    );

    // let mut some_var = 10;   // If not used anywhere, Rust throws error
    // let mut _some_var = 10;  // Rust will ignore that this variable is unused

    let mut player = Player::new(Vector2f::new((60.0) as f32, (115.0) as f32), 
                                         Vector2f::new(1.0, 1.0), 60.0);
    let map = Wall::new(2, 64, 64);


    // Player movement vars
    let mut up = false;
    let mut down = false;
    let mut right = false;
    let mut left = false;

    let mut clock = Clock::start();  // To calculate delta time
    // loop - infinite loop until  break;  or program exit  -  while(window.isOpen()) in C++ to catch that 
    loop {
        let delta_time = clock.restart().as_seconds();

        while let Some(event) = window.poll_event() {
            // swtich(event) {} in C++
            match event {
                Event::Closed | Event::KeyPressed {code: Key::Escape, ..} => return,
                
                // In C++ we've got Keyboard::isKeyPressed(Keyboard::A)
                // In Rust it's only event based - and events run in their loop
                // So I just set variable here and move player in main game loop
                Event::KeyPressed  {code: Key::W, ..} => up = true,
                Event::KeyReleased {code: Key::W, ..} => up = false,

                Event::KeyPressed  {code: Key::S, ..} => down = true,
                Event::KeyReleased {code: Key::S, ..} => down = false,

                Event::KeyPressed  {code: Key::D, ..} => right = true,
                Event::KeyReleased {code: Key::D, ..} => right = false,

                Event::KeyPressed  {code: Key::A, ..} => left = true,
                Event::KeyReleased {code: Key::A, ..} => left = false,

                _ => {}     // `default:` case in C++ `switch`
            }
        } window.clear(Color::rgb(80, 80, 80));


        if up    {player.advance( from_angle(radians(player.rotation)), delta_time, &map);}
        if down  {player.advance(-from_angle(radians(player.rotation)), delta_time, &map);}
        if right {player.rotate( 80.0 * delta_time);}
        if left  {player.rotate(-80.0 * delta_time);}
        

        // let mut camera = View::new(Vector2f::new(player.position.x, player.position.y), Vector2f::new(width as f32, height as f32));
        // camera.set_center(player.coordinates);
        // window.set_view(&camera);


        // Very similar to C++ function call: `map.draw(&window);`
        // map.draw(&mut window);
        // player.draw(&mut window);
        player.look(&map, &mut window);

        window.display();
    }
}