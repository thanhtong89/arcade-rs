// #[macro_use] asks the compiler to import the macros defined in the `events`
// module. This is necessary because macros cannot be namespaced -- macro
// expansion happens before the concept of namespace even starts to _exist_ in
// the compilation timeline.
#[macro_use]
mod events;
extern crate sdl2;

use sdl2::render::Renderer;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    }
    else: {
        quit: Quit { .. }
    }
}

// Bundles the Phi abstractions in a single structure which
// can be passed easily between functions.
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

impl<'a> Phi<'a> {
    pub fn new(sdl_context: sdl2::Sdl, window: sdl2::video::Window) -> Phi<'a> {
        Phi {
            events : Events::new(sdl_context.event_pump().unwrap()),
            renderer : window.renderer()
                            .accelerated()
                            .build().unwrap()
        }
    }
}

// A `ViewAction` is a way for the currently executed view to
// communicate with the game loop. It specifies which action
// should be executed before the next rendering.
pub enum ViewAction {
    None,
    Quit,
}

pub trait View {
    // Called on every frame to take care of both the logic and
    // the rendering of the current view.
    //
    // `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}
