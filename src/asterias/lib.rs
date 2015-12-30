#![crate_name = "asterias"]
#![crate_type = "lib"]

extern crate sdl2;
extern crate core;

use sdl2::video::{WindowPos, Window, OPENGL};
use sdl2::event::Event;
use std::vec::Vec;
use control::Control;
use std::time::Duration;

pub mod button;
pub mod control;

pub struct AstWindow {
	title: String,
	renderer: sdl2::render::Renderer,
	controls: Vec<Box<Control + 'static>>,
	background_color: sdl2::pixels::Color
}

impl AstWindow {
	pub fn new(title: &str, width: i32, height: i32) -> AstWindow {

		let window = match Window::new(title, WindowPos::PosCentered, WindowPos::PosCentered, width, height, OPENGL) {
	        Ok(window) => window,
	        Err(err)   => panic!("failed to create window: {}", err)
    	};

		let retval = AstWindow {
			title: title.to_string(),
			renderer: match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
		        Ok(renderer) => renderer,
		        Err(err) => panic!("failed to create renderer: {}", err)
		    },
		    controls: Vec::new(),
		    background_color: sdl2::pixels::Color::RGB(200, 200, 200)
		};

        let mut drawer = retval.renderer.drawer();

	    let _ = drawer.set_draw_color(retval.background_color);

	    // Clear the buffer, using the light blue color set above.
	    let _ = drawer.clear();

	    // Swap our buffer for the present buffer, displaying it.
	    let _ = drawer.present();

	    return retval;
	}

	pub fn add_control(&mut self, control: Box<Control + 'static>) {
		self.controls.push(control)
	}

	pub fn draw(&mut self) {
		let mut present: bool = false;
		let mut i = 0;
		let length = self.controls.len();

        let mut drawer = self.renderer.drawer();
	    let _ = drawer.set_draw_color(self.background_color);

	    // Clear the buffer, using the light blue color set above.
	    let _ = drawer.clear();
		while i < length {
			present |= self.controls[i].draw(&self.renderer);
			i += 1;
		}
		if present {
			drawer.present();
		}
	}
}

pub struct Asterias {
    window_list: Vec<AstWindow>,
    sdl_context: sdl2::sdl::Sdl
}

impl Asterias {
	pub fn new() -> Asterias {
		let mut retval = Asterias {
			window_list: Vec::new(),
            sdl_context: sdl2::init(sdl2::INIT_EVERYTHING).unwrap() 
		};
		return retval;
	}

	pub fn add_window(&mut self, window: AstWindow) {
		self.window_list.push(window);
	}

	//this is the 'main' function.
	//each call handles input, rendering, and presentation
	pub fn step_ui(&mut self) -> bool {

		self.framepacer.recv();

		let mut i = 0;
		let length = self.window_list.len();
		while i < length {
			self.window_list[i].draw();
			i += 1;
		}
        let mut event_pump = self.sdl_context.event_pump();
        for event in event_pump.poll_iter() {
            let quit = match event {
                Event::Quit{..} => true,
                _ => false
            };
            if quit {
                return true;
            }
        }
        return false;
	}
}
