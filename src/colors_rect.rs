use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use palette::{Hsv, Srgb, IntoColor};

use crate::config::COLORS_RECT_POINT_SIZE;
use crate::tools::{get_rect_center, set_rect_center};

pub struct ColorRect {
    rect: Rect,
    point: Point,
    point_color: Color,
    hue: f32, // hue
}

impl ColorRect {
    pub fn new(rect: Rect) -> ColorRect {
        let mut cr = ColorRect {
            rect: rect,
            point: Point::new(0, 0),
            point_color: Color::RGB(0, 0, 0),
            hue: 120.,
        };
        cr.point.x = cr.rect.x;
        cr.point.y = cr.rect.y;
        cr
    }

    fn draw_point(&mut self, canvas: &mut Canvas<Window>) {
        let mut rect = Rect::new(0, 0, COLORS_RECT_POINT_SIZE, COLORS_RECT_POINT_SIZE);

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
        let w = self.rect.w / 100;
        let h = self.rect.h / 100;
        for s in 0..100 {
            for v in 0..100 {
                let rgb: Srgb = Hsv::new(
                    self.hue,
                    s as f32 / 100.,
                    v as f32 / 100.
                ).into_color();
                let color = Color::RGB(
                    (rgb.red * 255.) as u8,
                    (rgb.green * 255.) as u8,
                    (rgb.blue * 255.) as u8,
                );

                let x = self.rect.x + s * w;
                let y = self.rect.y + (100 - v) * h;

                if (self.point.x - x).abs() <= 5 && (self.point.y - y).abs() <= 5 {
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
