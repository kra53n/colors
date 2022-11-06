use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use palette::{Hsv, Srgb, IntoColor};

use crate::config::COLORS_RECT_POINT_SIZE;
use crate::tools::{get_rect_center, set_rect_center};

pub struct ColorsLine {
    rect: Rect,
    point: Point,
}

impl ColorsLine {
    pub fn new(rect: Rect) -> ColorsLine {
        let mut cl = ColorsLine {
            rect: rect,
            point: Point::new(0, 0),
        };
        cl.point.x = cl.rect.x;
        cl.point.y = cl.rect.y;
        cl
    }


    // fn draw_point(&mut self, canvas: &mut Canvas<Window>) {
    //     let mut rect = Rect::new(0, 0, COLORS_RECT_POINT_SIZE, COLORS_RECT_POINT_SIZE);

    //     let colors = [Color::RGB(0, 0, 0), self.point_color];
    //     for color in colors {
    //         set_rect_center(&mut rect, &mut self.point);

    //         canvas.set_draw_color(color);
    //         canvas.fill_rect(rect).expect("can't draw fill rect");

    //         rect.w -= 2;
    //         rect.h -= 2;
    //     }
    // }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let w: u32 = 1;
        let mut rect: Rect = Rect::new(self.rect.x, self.rect.y, w, self.rect.h as u32);
        for i in 0..(self.rect.w / w as i32) {
            let rgb: Srgb = Hsv::new((i * 360 * w as i32 / self.rect.w) as f32, 1., 1.).into_color();
            let color = Color::RGB(
                (rgb.red * 255.) as u8,
                (rgb.green * 255.) as u8,
                (rgb.blue * 255.) as u8,
            );

            canvas.set_draw_color(color);
            canvas.draw_rect(rect).expect("can't draw rect");
            rect.x += w as i32;
        }
    }
}
