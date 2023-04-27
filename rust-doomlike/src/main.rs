mod renderer;
use crate::renderer::Renderer;
use crate::renderer::player::Player;
use crate::renderer::texture::Texture as HkkTexture;

use sfml::{
    // audio::{Sound, SoundBuffer, SoundSource},
    system::{Vector2f},
    window::{ContextSettings, Event, Key, Style},
    graphics::{Color, RenderTarget, RenderWindow, View},
};


const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const RENDER_W: f32 = 320.0;
const RENDER_H: f32 = 240.0;

const FPS: u32 = 24;
const VELOCITY: i32 = 4;
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


    loop {
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
        if right {player.advance(Key::D, VELOCITY);}
        if left  {player.advance(Key::A, VELOCITY);}
        if strafe_left  {player.advance(Key::Left,  VELOCITY);}
        if strafe_right {player.advance(Key::Right, VELOCITY);}
        if move_up   {player.advance(Key::Q, VELOCITY);}
        if move_down {player.advance(Key::E, VELOCITY);}
        if look_up   {player.advance(Key::Up,   VELOCITY);}
        if look_down {player.advance(Key::Down, VELOCITY);} 


        // let textures = HkkTexture::texture_loader();
        // HkkTexture::test_textures(1, textures, &mut window);
        // renderer.floor(&player, &mut window);
        renderer.draw(&player, &mut window);
        window.display();
    }
}
