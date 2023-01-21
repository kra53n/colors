extern crate sdl2;

use std::error::Error;
use std::time::Duration;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use tools::{
	hsv2rgb,
	ColorsRect,
	ColorsLine,
	ColorSquare,
};

use tools::traits::{
	Draw,
};

pub struct Config {
	pub title: &'static str,
	pub w: u32,
	pub h: u32,
	pub font_path: &'static str,
	pub font_size: u16,
	pub color_rect_point_size: u32,
	pub color_square_border_size: i32,
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem.window(config.title, config.w, config.h)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let font = ttf_context.load_font(config.font_path, config.font_size).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut colors_rect: ColorsRect = ColorsRect::new(
		Rect::new(20, 20, 400, 200),
		config.color_rect_point_size,
	);
    let mut colors_line: ColorsLine = ColorsLine::new(
		Rect::new(20, 236, 400, 8),
		config.color_rect_point_size,
	);
    let mut color_square: ColorSquare = ColorSquare::new(
			Rect::new(
			436 + config.color_square_border_size as i32 / 2,
			20 + config.color_square_border_size as i32 / 2, 80, 80
		),
        Color::RGB(255, 255, 255),
		config.color_square_border_size,
    );

    let surface = font
		.render("Hello sdl2_ttf!")
		.blended(Color::RGB(255, 0, 0))
		.unwrap();

    let mut to_draw = true;

    'running: loop {
        if to_draw {
			canvas.set_draw_color(Color::RGB(26, 27, 38));
			canvas.clear();

			colors_rect.draw(&mut canvas);
			colors_line.draw(&mut canvas);
			color_square.draw(&mut canvas);

			canvas.present();
        }
        to_draw = false;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {
                    to_draw = true;
                },
            }
        }

		let mouse = event_pump.mouse_state();
		colors_rect.update(&mouse);
		colors_line.update(&mouse);
		if mouse.is_mouse_button_pressed(MouseButton::Left) {
			colors_rect.set_hue(colors_line.get_hue());
			color_square.set_color(
				hsv2rgb(
					colors_line.get_hue(),
					colors_rect.get_saturation(),
					colors_rect.get_value()
				)
			);
		}

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

	Ok(())
}
