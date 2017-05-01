use sdl2::pixels::Color;
use phi::{Phi, View, ViewAction};
pub struct DefaultView;

impl View for DefaultView {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction {
        if context.events.now.quit || context.events.now.key_escape == Some(true) {
            ViewAction::Quit
        }
        else {
            context.renderer.set_draw_color(Color::RGB(0, 0, 0));
            context.renderer.clear();
            ViewAction::None
        }
    }
}
