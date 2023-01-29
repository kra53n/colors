use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::mouse::MouseState;
use sdl2::rect::{Rect, Point};

use crate::traits::Component;
use crate::funcs::{set_rect_center, hsv2rgb, return_point_to_rect_edge};

pub struct ColorsRect {
    rect: Rect,
    point: Point,
    point_color: Color,
	point_size: u32,
    hue: f32,
    toggled: bool,
}

impl ColorsRect {
    pub fn new(rect: Rect, point_size: u32) -> ColorsRect {
        let mut cr = ColorsRect {
            rect,
            point: Point::new(0, 0),
            point_color: Color::RGB(0, 0, 0),
			point_size,
            hue: 0.,
			toggled: false,
        };
        cr.point.x = cr.rect.x;
        cr.point.y = cr.rect.y;
        cr
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


    pub fn update(&mut self, mouse: &MouseState) {
		let cursor = Point::new(mouse.x(), mouse.y());
		if (mouse.left() && self.rect.contains_point(cursor)) || self.toggled {
			self.point = cursor;
			self.toggled = true;
			return_point_to_rect_edge(&mut self.point, self.rect);
		}

		if !mouse.left() {
			self.toggled = false;
		}
    }

    pub fn set_hue(&mut self, hue: f32) {
	self.hue = hue;
    }

    pub fn get_saturation(&self) -> f32 {
	return (self.point.x - self.rect.x) as f32 / self.rect.w as f32;
    }

    pub fn get_value(&self) -> f32 {
	return 1. - (self.point.y - self.rect.y) as f32 / self.rect.h as f32;
    }
}

impl Component for ColorsRect {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let w = self.rect.w / 100;
        let h = self.rect.h / 100;
        for s in 0..100 {
            for v in 0..100 {
				let color = hsv2rgb(self.hue, s as f32 / 100., v as f32 / 100.);

                let x = self.rect.x + s * w;
                let y = self.rect.y + (100 - v) * h;

                if (self.point.x - x).abs() <= 2 && (self.point.y - y).abs() <= 2 {
                    self.point_color = color;
                }

                let rect = Rect::new(x, y, w as u32, h as u32);

                canvas.set_draw_color(color);
                canvas.fill_rect(rect).expect("can't fill rect");
            }
        }
        self.draw_point(canvas);
    }
}
