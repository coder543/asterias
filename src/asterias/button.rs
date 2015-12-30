extern crate sdl2;
use sdl2::rect::Rect;
use control::Control;

pub struct Button {
	dirty: bool,
	region: Rect
}

impl Copy for Button {}

impl Button {
	pub fn new_from_rect(region: Rect) -> Button {
		Button {
			region: region,
			dirty: true
		}
	}

	pub fn new_from_location(x: i32, y: i32) -> Button {
		Button {
			region: Rect::new(x,y,150,30),
			dirty: true
		}
	}
}

impl Control for Button {
	fn draw(&mut self, renderer: &sdl2::render::Renderer) -> bool {
		if !self.dirty {
			return false;
		}
		self.dirty = false;
        let mut drawer = renderer.drawer();
	    // Set the drawing color to a darker blue.
	    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
	    // Create centered Rect, draw the outline of the Rect in our dark blue color.
	    drawer.fill_rect(self.region);
	    return true;
	}
}
