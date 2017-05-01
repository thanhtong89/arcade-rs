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
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
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
    ChangeView(Box<View>),
}

pub trait View {
    // Called on every frame to take care of both the logic and
    // the rendering of the current view.
    //
    // `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}
/// Create a window with name `title`, initialize the underlying libraries and
/// start the game with the `View` returned by `init()`.
///
/// # Examples
///
/// Here, we simply show a window with color #ffff00 and exit when escape is
/// pressed or when the window is closed.
///
/// ```
/// struct MyView;
///
/// impl View for MyView {
///     fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
///         if context.events.now.quit {
///             return ViewAction::Quit;
///         }
///
///         context.renderer.set_draw_color(Color::RGB(255, 255, 0));
///         context.renderer.clear();
///         ViewAction::None
///     }
/// }
///
/// spawn("Example", |_| {
///     Box::new(MyView)
/// });
/// ```
pub fn spawn<F>(title: &str, init: F)
where F: Fn(&mut Phi) -> Box<View> {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    // Create context
    let mut context = Phi::new(sdl_context, window);

    // Create the default view
    let mut current_view = init(&mut context);

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
            ViewAction::None => context.renderer.present(),
            ViewAction::ChangeView(new_view) =>
                current_view = new_view,
        }
    }
}
