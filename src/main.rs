use std::process;

fn main() {
	let config = mycolor::Config {
		title: "mycolor",
		w: 800,
		h: 600,
		font_path: "assets/fonts/JetBrainsMono-Regular.ttf",
		font_size: 16,
		color_rect_point_size: 16,
		color_square_border_size: 4,
	};

	if let Err(e) = mycolor::run(&config) {
		println!("Application error: {}", e);
		process::exit(1);
	}
}
