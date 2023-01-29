use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

use crate::traits::Component;

pub struct ColorSquare {
    rect: Rect,
    color: Color,
	border_size: i32,
}

impl ColorSquare {
    pub fn new(rect: Rect, color: Color, border_size: i32) -> ColorSquare {
        ColorSquare {
            rect,
            color,
			border_size,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Component for ColorSquare {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let rects = [
            Rect::new(
				self.rect.x - self.border_size / 2,
				self.rect.y - self.border_size / 2,
				(self.rect.w + self.border_size) as u32,
				(self.rect.h + self.border_size) as u32,
            ),
            self.rect,
        ];
        let colors = [Color::RGB(0, 0, 0), self.color];
        for (&rect, color) in rects.iter().zip(colors) {
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).expect("can't fill rect");
        }
    }
}
