use sdl2::video::Window;
use sdl2::render::Canvas;

pub trait Component {
	fn draw(&mut self, canvas: &mut Canvas<Window>);
}
