use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::rect::{Rect, Point};

struct Label<'a> {
    texture: &'a Texture<'a>,
    rect: Rect,
    text: &'a str,
    text_col: Color,
    label_col: Color,
}

impl<'a> Label<'a> {
    pub fn new(
	x: i32, y: i32, text: &'a str, text_col: Color,
	label_col: Color
    ) -> Label<'a>
    {
	Label {
	    texture: texture,
	    rect: rect,
	    text: text,
	    text_col: text_col,
	    label_col: label_col,
	}
    }

    pub fn draw(&self) {
    }

    pub fn set_text(&mut self, text: &'a str) {
	self.text = text;
    }

    pub fn get_text(&self) -> &'a str {
	self.text
    }
}
