extern crate sdl2;

mod phi;
mod views;

use phi::{Phi, View, ViewAction};
use views::DefaultView;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    // create context
    let mut context = Phi::new(sdl_context, window);
    let mut current_view = Box::new(DefaultView);

    // Frame timimg
    let interval = 1000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    loop {
        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1000.0;

        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;
        if now - last_second > 1000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }
        context.events.pump();
        match (*current_view).render(&mut context, elapsed) {
            ViewAction::Quit => break,
            ViewAction::None => context.renderer.present()
        }
    }
}
