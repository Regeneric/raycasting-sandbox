mod renderer;
use crate::renderer::Renderer;
use crate::renderer::player::Player;

use sfml::{
    system::{Vector2f, Clock},
    window::{ContextSettings, Event, Key, Style},
    graphics::{Color, RenderTarget, RenderWindow, View, Text, Font, Transformable},
};


const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const RENDER_W: f32 = 160.0;
const RENDER_H: f32 = 120.0;

const FPS: u32 = 30;
const VELOCITY: f32 = 4.0;
const FOV: i32 = 200;


fn main() {
    let context_settings = ContextSettings {..Default::default()};
    let mut window = RenderWindow::new(
        (WIDTH, HEIGHT),
        "Rust Doomlike",
        Style::CLOSE,
        &context_settings,
    ); 
    
    // Scale render resolution to window resolution
    let mut viewport = View::new(Vector2f::new(RENDER_W/2.0, RENDER_H/2.0), Vector2f::new(RENDER_W, RENDER_H));
    viewport.set_rotation(180.0);
    window.set_view(&viewport);
    window.set_framerate_limit(FPS);

    let mut player = Player::new();
    let mut renderer = Renderer::new();

    let (mut up, mut right, mut down, mut left, mut strafe_left, mut strafe_right, mut move_up, mut move_down, mut look_up, mut look_down) 
      = (false, false, false, false, false, false, false, false, false, false);

    let mut clock = Clock::start();
    loop {
        let delta_time = clock.elapsed_time().as_seconds();
        clock.restart();

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed {code: Key::Escape, ..} => return,

                Event::KeyPressed  {code: Key::W, ..} => up = true,
                Event::KeyReleased {code: Key::W, ..} => up = false,

                Event::KeyPressed  {code: Key::S, ..} => down = true,
                Event::KeyReleased {code: Key::S, ..} => down = false,

                Event::KeyPressed  {code: Key::D, ..} => right = true,
                Event::KeyReleased {code: Key::D, ..} => right = false,

                Event::KeyPressed  {code: Key::A, ..} => left = true,
                Event::KeyReleased {code: Key::A, ..} => left = false,

                Event::KeyPressed  {code: Key::Left, ..}  => strafe_left = true,
                Event::KeyReleased {code: Key::Left, ..}  => strafe_left = false,

                Event::KeyPressed  {code: Key::Right, ..} => strafe_right = true,
                Event::KeyReleased {code: Key::Right, ..} => strafe_right = false,

                Event::KeyPressed  {code: Key::Q, ..}  => move_up = true,
                Event::KeyReleased {code: Key::Q, ..}  => move_up = false,

                Event::KeyPressed  {code: Key::E, ..}  => move_down = true,
                Event::KeyReleased {code: Key::E, ..}  => move_down = false,

                Event::KeyPressed  {code: Key::Up, ..} => look_up = true,
                Event::KeyReleased {code: Key::Up, ..} => look_up = false,

                Event::KeyPressed  {code: Key::Down, ..} => look_down = true,
                Event::KeyReleased {code: Key::Down, ..} => look_down = false,

                _ => {}
            }
        } window.clear(Color::rgb(80, 100, 80));

        if up    {player.advance(Key::W, VELOCITY);}
        if down  {player.advance(Key::S, VELOCITY);}
        if right {player.advance(Key::D, VELOCITY * delta_time);}
        if left  {player.advance(Key::A, VELOCITY * delta_time);}
        if strafe_left  {player.advance(Key::Left,  VELOCITY);}
        if strafe_right {player.advance(Key::Right, VELOCITY);}
        if move_up   {player.advance(Key::Q, VELOCITY * delta_time);}
        if move_down {player.advance(Key::E, VELOCITY * delta_time);}
        if look_up   {player.advance(Key::Up,   VELOCITY);}
        if look_down {player.advance(Key::Down, VELOCITY);} 

        renderer.draw(&player, &mut window);

        let arial = Font::from_file("src/fonts/arial.ttf").unwrap();
        let mut pos_x = Text::new(&["X: ", &player.pos.x.to_string()].join(""), &arial, 18); 
            pos_x.set_position(Vector2f::new(15.0, 20.0)); 
            pos_x.set_scale(Vector2f::new(-0.25, -0.25));

        let mut pos_y = Text::new(&["Y: ", &player.pos.y.to_string()].join(""), &arial, 18); 
            pos_y.set_position(Vector2f::new(15.0, 15.0)); 
            pos_y.set_scale(Vector2f::new(-0.25, -0.25));
        
        let mut pos_z = Text::new(&["Z: ", &player.pos.z.to_string()].join(""), &arial, 18); 
            pos_z.set_position(Vector2f::new(15.0, 10.0)); 
            pos_z.set_scale(Vector2f::new(-0.25, -0.25));

        window.draw(&pos_x);
        window.draw(&pos_y);
        window.draw(&pos_z);

        window.display();
    }
}
