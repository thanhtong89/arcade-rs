use sdl2::pixels::Color;
use phi::{Phi, View, ViewAction};
pub struct ViewA;
pub struct ViewB;

fn render_template(context: &mut Phi, elapsed: f64, color: Color,
                    next_view: Box<View>) -> ViewAction {
        if context.events.now.quit ||
            context.events.now.key_escape == Some(true) {
            ViewAction::Quit
        }
        else if context.events.now.key_space == Some(true) {
            ViewAction::ChangeView(next_view)
        }
        else {
            context.renderer.set_draw_color(color);
            context.renderer.clear();
            ViewAction::None
        }
}

impl View for ViewA {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction {
        render_template(context, elapsed, Color::RGB(255, 0, 0), Box::new(ViewB))
    }
}

impl View for ViewB {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction {
        render_template(context, elapsed, Color::RGB(0, 0, 255), Box::new(ViewA))
    }
}
