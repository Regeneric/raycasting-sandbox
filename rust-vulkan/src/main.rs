#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod app;
use crate::app::App;

use anyhow     :: Result;
use winit      :: dpi        :: LogicalSize;
use winit      :: event      :: {Event, WindowEvent};
use winit      :: event_loop :: {ControlFlow, EventLoop};
use winit      :: window     :: WindowBuilder;
use vulkanalia :: prelude    :: v1_0 :: *;


fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Vulkan")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    let mut app = unsafe {App::create(&window)?};
    let mut destroying = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Event handle - like in any other lib (eg. SFML)
        match event {
            Event::MainEventsCleared if !destroying => {
                unsafe {app.render(&window)}.unwrap();
            },
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe {app.device.device_wait_idle().unwrap();}
                unsafe {app.destroy();}
            }
            _ => {}
        }
    });
}   