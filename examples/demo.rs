#![feature(box_syntax)]
extern crate sdl2;
extern crate asterias;
use asterias::Asterias;
use asterias::AstWindow;
use asterias::button::Button;

fn main() {
	let mut asteria = Asterias::new();
	let mut window = AstWindow::new("An Excellent Demo", 640, 480);
	window.add_control(box Button::new_from_location(290, 430));
	window.add_control(box Button::new_from_location(470, 430));
	asteria.add_window(window);
	while asteria.step_ui() {};
}
