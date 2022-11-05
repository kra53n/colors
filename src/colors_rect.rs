use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use palette::{Hsv, Srgb, IntoColor};

pub struct ColorRect {
    rect: Rect,
    point: Point,
    hue: f32, // hue
}

impl ColorRect {
    pub fn new(rect: Rect) -> ColorRect {
        ColorRect {
            rect: rect,
            point: Point::new(0, 0),
            hue: 120.,
        }
    }

    fn draw_point(&mut self, canvas: &mut Canvas<Window>) {

    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let w = self.rect.w / 100;
        let h = self.rect.h / 100;
        for s in 0..100 {
            for v in 0..100 {
                let rect = Rect::new(
                    (self.rect.x + s * w) as i32,
                    (self.rect.y + (100 - v) * h) as i32,
                    w as u32,
                    h as u32
                );

                let rgb: Srgb = Hsv::new(
                    self.hue,
                    s as f32 / 100.,
                    v as f32 / 100.
                ).into_color();

                canvas.set_draw_color(Color::RGB(
                    (rgb.red * 255.) as u8,
                    (rgb.green * 255.) as u8,
                    (rgb.blue * 255.) as u8,
                ));

                canvas.fill_rect(rect).expect("can't fill rect");
            }
        }
    }
}
