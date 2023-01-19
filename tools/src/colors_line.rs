use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::mouse::MouseState;
use sdl2::rect::{Rect, Point};

use crate::funcs::{hsv2rgb, set_rect_center, return_point_to_rect_edge};

pub struct ColorsLine {
    rect: Rect,
    point: Point,
    point_color: Color,
	point_size: u32,
    toggled: bool,
}

impl ColorsLine {
    pub fn new(rect: Rect, point_size: u32) -> ColorsLine {
        let mut cl = ColorsLine {
            rect: rect,
            point_color: Color::RGB(0, 0, 0),
            point: Point::new(0, 0),
			point_size,
            toggled: false,
        };
        cl.point.x = cl.rect.x;
        cl.point.y = cl.rect.y + cl.rect.h / 2;
        cl
    }


    fn draw_point(&mut self, canvas: &mut Canvas<Window>) {
        let mut rect = Rect::new(0, 0, self.point_size, self.point_size);

        let colors = [Color::RGB(0, 0, 0), self.point_color];
        for color in colors {
            set_rect_center(&mut rect, &mut self.point);

            canvas.set_draw_color(color);
            canvas.fill_rect(rect).expect("can't draw fill rect");

            rect.w -= 2;
            rect.h -= 2;
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let w: u32 = 1;
        let mut rect: Rect = Rect::new(self.rect.x, self.rect.y, w, self.rect.h as u32);
        for i in 0..(self.rect.w / w as i32) {
	    let color = hsv2rgb((i * 360 * w as i32 / self.rect.w) as f32, 1., 1.);

	    if self.point.x - self.rect.x == i {
		self.point_color = color;
	    }

            canvas.set_draw_color(color);
            canvas.draw_rect(rect).expect("can't draw rect");
            rect.x += w as i32;
        }
        self.draw_point(canvas);
    }

    pub fn update(&mut self, mouse: &MouseState) {
	let cursor = Point::new(mouse.x(), mouse.y());
	if (mouse.left() && self.rect.contains_point(cursor)) || self.toggled {
	    self.point = cursor;
	    self.toggled = true;
	    return_point_to_rect_edge(&mut self.point, self.rect);
	    self.point.y = self.rect.y + self.rect.h / 2;
	}

	if !mouse.left() {
	    self.toggled = false;
	}
    }

    pub fn get_hue(&self) -> f32 {
	return (self.point.x - self.rect.x) as f32 * 360. / self.rect.w as f32;
    }
}
