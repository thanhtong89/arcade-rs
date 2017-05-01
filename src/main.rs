extern crate sdl2;
extern crate sdl2_image;

mod phi;
mod views;

fn main() {
    ::phi::spawn("ArcadeRS Shooter", |context| {
        Box::new(::views::ShipView::new(context))
    });
}
