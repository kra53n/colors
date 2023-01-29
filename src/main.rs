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

	mycolor::App::init(config)
		.expect("something")
		.run();
}
