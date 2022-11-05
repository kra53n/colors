use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Rect, Point};

pub struct ColorRect {
    rect: Rect,
    point: Point,
}

impl ColorRect {
    pub fn new(rect: Rect) -> ColorRect {
        let cr = ColorRect {
            rect: rect,
            point: Point::new(0, 0),
        };
        cr
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        canvas.fill_rect(self.rect);
    }
}
