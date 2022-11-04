use sdl2::rect::{Rect, Point};

pub struct ColorRect {
    rect: Rect,
    point: Point,
}

impl ColorRect {
    pub fn new(rect: Rect) -> ColorRect {
        let mut cr = ColorRect {
            rect: rect,
            point: Point::new(0, 0),
        };
        cr
    }
}
