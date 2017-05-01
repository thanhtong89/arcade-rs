use sdl2::pixels::Color;
use phi::{Phi, View, ViewAction};
pub struct ViewA;
pub struct ViewB;

impl View for ViewA {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction {
        if context.events.now.quit ||
            context.events.now.key_escape == Some(true) {
            ViewAction::Quit
        }
        else if context.events.now.key_space == Some(true) {
            ViewAction::ChangeView(Box::new(ViewB))
        }
        else {
            context.renderer.set_draw_color(Color::RGB(255, 0, 0));
            context.renderer.clear();
            ViewAction::None
        }
    }
}

impl View for ViewB {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction {
        if context.events.now.quit ||
            context.events.now.key_escape == Some(true) {
            ViewAction::Quit
        }
        else if context.events.now.key_space == Some(true) {
            ViewAction::ChangeView(Box::new(ViewA))
        }
        else {
            context.renderer.set_draw_color(Color::RGB(0, 0, 255));
            context.renderer.clear();
            ViewAction::None
        }
    }
}
