#![crate_name = "asterias"]
#![crate_type = "lib"]

use std::vec::Vec;

pub struct Window {
	title: String
}

impl Window {
	pub fn new(title: String) -> Window {
		Window {
			title: title
		}
	}
}

pub struct Asterias {
    window_list: Vec<Window>
}

impl Asterias {
	pub fn new() -> Asterias {
		Asterias {
			window_list: Vec::new()
		}
	}

	pub fn add_window(&mut self, window: Window) {
		self.window_list.push(window);
	}

	//this is the 'main' function.
	//each call handles input, rendering, and presentation
	pub fn step_ui(&mut self) {
		println!("Windows currently managed: ");
		for window in self.window_list.iter() {
			println!("{}", window.title);
		}
	}
}