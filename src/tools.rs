use sdl2::rect::{Rect, Point};

pub fn get_rect_center(rect: Rect) -> Point {
    Point::new(rect.x + rect.w / 2, rect.y + rect.h / 2)
}

pub fn set_rect_center(rect: &mut Rect, point: &mut Point) {
    rect.x = point.x - rect.w / 2;
    rect.y = point.y - rect.h / 2;
}
