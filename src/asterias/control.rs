extern crate sdl2;
pub trait Control {
	fn draw(&mut self, renderer: &sdl2::render::Renderer) -> bool;
}