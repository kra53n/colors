use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

struct Label<'ttf> {
    font: &'ttf Font<'ttf, 'ttf>,
    rect: Rect,
    text_col: Color,
    label_col: Color,
}

impl<'ttf> Label<'ttf> {
    pub fn new(font: &'ttf Font, rect: Rect, text_col: Color, label_col: Color) -> Label<'ttf> {
	Label {
	    font: font,
	    rect: rect,
	    text_col: text_col,
	    label_col: label_col,
	}
    }
}
