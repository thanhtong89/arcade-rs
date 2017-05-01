extern crate sdl2;

mod phi;
mod views;

fn main() {
    ::phi::spawn("ArcadeRS Shooter", |context| {
        Box::new(::views::ShipView::new(context))
    });
}
