use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

struct Label {
    font: Font,
    rect: Rect,
    text_col: Color,
    label_col: Color,
}

impl Label {
    pub fn new(font: Font, rect: Rect, text_col: Color, label_col: Color) {
	Label(
	    font: font,
	    rect: rect,
	    text_col: text_col,
	    label_col: label_col,
	)
    }
}
