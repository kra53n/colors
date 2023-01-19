use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use palette::{Hsv, Srgb, IntoColor};

pub fn set_rect_center(rect: &mut Rect, point: &mut Point) {
    rect.x = point.x - rect.w / 2;
    rect.y = point.y - rect.h / 2;
}

pub fn return_point_to_rect_edge(point: &mut Point, rect: Rect) {
    if point.x > rect.x + rect.w {
	point.x = rect.x + rect.w;
    }
    if point.x < rect.x {
	point.x = rect.x;
    }

    if point.y > rect.y + rect.h {
	point.y = rect.y + rect.h;
    }
    if point.y < rect.y {
	point.y = rect.y;
    }
}

/// arugemnts:
/// h: 0. - 360.
/// s: 0. - 1.
/// v: 0. - 1.
pub fn hsv2rgb(h: f32, s: f32, v: f32) -> Color {
    let rgb: Srgb = Hsv::new(h, s, v).into_color();
    return Color::RGB(
	(rgb.red * 255.) as u8,
	(rgb.green * 255.) as u8,
	(rgb.blue * 255.) as u8,
    );
}
