use eyre::Result;
use ggez::{event, ContextBuilder};
use ggez::conf::{WindowSetup, WindowMode};
use ggez_rust::MainState;


const GAME_NAME: &str = "GGEZ Rust Raycaster";
const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;
const V_SYNC: bool = false;
const RESIZABLE: bool = false;
const MAXIMIZED: bool = false;


fn main() -> Result<()> {
    let window_setup = WindowSetup::default()
                                    .title(GAME_NAME)
                                    .vsync(V_SYNC);

    let window_mode = WindowMode::default()
                                    .dimensions(WIDTH, HEIGHT)
                                    .resizable(RESIZABLE)
                                    .maximized(MAXIMIZED);

    let (mut context, event_loop) = ContextBuilder::new("hkk-raycast", "hkk")
                                                        .backend(ggez::conf::Backend::Vulkan)
                                                        .window_mode(window_mode)
                                                        .window_setup(window_setup)
                                                        .build()?;
    let mut main_state = MainState::new(GAME_NAME);
    
    main_state.setup(&mut context)?;
    event::run(context, event_loop, main_state)
}
