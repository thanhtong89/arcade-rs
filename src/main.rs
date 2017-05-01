extern crate sdl2;

// #[macro_use] asks the compiler to import the macros defined in the `events`
// module. This is necessary because macros cannot be namespaced -- macro
// expansion happens before the concept of namespace even starts to _exist_ in
// the compilation timeline.
#[macro_use]
mod phi;
mod views;

use sdl2::pixels::Color;
use phi::Events;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();


    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    // Prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.now.quit || events.now.key_escape == Some(true) {
            break;
        }
        // Render a black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
