use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

use crate::config::COLOR_SQUARE_BORDER_SIZE;

pub struct ColorSquare {
    rect: Rect,
    color: Color,
}

impl ColorSquare {
    pub fn new(rect: Rect, color: Color) -> ColorSquare {
	ColorSquare {
	    rect: rect,
	    color: color,
	}
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
	let rects = [
	    Rect::new(
		self.rect.x - COLOR_SQUARE_BORDER_SIZE as i32 / 2,
		self.rect.y - COLOR_SQUARE_BORDER_SIZE as i32 / 2,
		self.rect.w as u32 + COLOR_SQUARE_BORDER_SIZE,
		self.rect.h as u32 + COLOR_SQUARE_BORDER_SIZE,
	    ),
	    self.rect,
	];
	let colors = [Color::RGB(0, 0, 0), self.color];
	for (&rect, color) in rects.iter().zip(colors) {
	    canvas.set_draw_color(color);
	    canvas.fill_rect(rect).expect("can't fill rect");
	}
    }

    pub fn set_color(&mut self, color: Color) {
	self.color = color;
    }
}
