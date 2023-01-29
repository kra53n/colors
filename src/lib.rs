extern crate sdl2;

use std::error::Error;
use std::time::Duration;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::render::Canvas;

use tools::{
	hsv2rgb,
	ColorsRect,
	ColorsLine,
	ColorSquare,
};

use tools::traits::{
	Component,
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

pub struct Context {
	sdl_context: Sdl,
	video_subsystem: VideoSubsystem,
	ttf_context: Sdl2TtfContext,
}

pub struct App {
	config: Config,
	context: Context,
	canvas: Canvas<Window>,
	event_pump: EventPump,
	//components: Vec<Box<dyn Component>>,
	to_draw: bool,
}

impl App {
	pub fn init(config: Config) -> Result<Self, Box<dyn Error>> {
		let sdl_context = sdl2::init().unwrap();
		let video_subsystem = sdl_context.video().unwrap();
		let ttf_context = sdl2::ttf::init().unwrap();
		let window = video_subsystem.window(config.title, config.w, config.h)
			.position_centered()
			.build()
			.unwrap();
		let canvas = window.into_canvas().build().unwrap();
		let event_pump = sdl_context.event_pump().unwrap();

		let context = Context {
			sdl_context,
			video_subsystem,
			ttf_context,
		};

		Ok(App {
			config,
			context,
			canvas,
			event_pump,
			to_draw: true,
		})
	}

	pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
		let mut colors_rect: ColorsRect = ColorsRect::new(
			Rect::new(20, 20, 400, 200),
			self.config.color_rect_point_size,
		);
		let mut colors_line: ColorsLine = ColorsLine::new(
			Rect::new(20, 236, 400, 8),
			self.config.color_rect_point_size,
		);
		let mut color_square: ColorSquare = ColorSquare::new(
				Rect::new(
				436 + self.config.color_square_border_size as i32 / 2,
				20 + self.config.color_square_border_size as i32 / 2, 80, 80
			),
			Color::RGB(255, 255, 255),
			self.config.color_square_border_size,
		);

		'running: loop {
			if self.to_draw {
				self.canvas.set_draw_color(Color::RGB(26, 27, 38));
				self.canvas.clear();

				colors_rect.draw(&mut self.canvas);
				colors_line.draw(&mut self.canvas);
				color_square.draw(&mut self.canvas);

				self.canvas.present();
			}
			self.to_draw = false;

			for event in self.event_pump.poll_iter() {
				match event {
					Event::Quit {..} |
					Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
						break 'running
					},
					_ => {
						self.to_draw = true;
					},
				}
			}

			let mouse = self.event_pump.mouse_state();
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
}
