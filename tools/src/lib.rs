mod funcs;
pub use funcs::{
	hsv2rgb,
	set_rect_center,
	return_point_to_rect_edge
};

pub mod traits;

mod colors_rect;
pub use colors_rect::ColorsRect;

mod colors_line;
pub use colors_line::ColorsLine;

mod color_square;
pub use color_square::ColorSquare;
