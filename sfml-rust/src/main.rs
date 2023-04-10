use sfml::{
    // audio::{Sound, SoundBuffer, SoundSource},
    graphics::{
        CircleShape, Color, RectangleShape, RenderTarget, RenderWindow, Shape,
        Transformable,
    },
    system::{Vector2f},
    window::{ContextSettings, Event, Key, Style},
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
        } 
        
        window.clear(Color::rgb(80, 80, 80));
        window.display();
    }
}