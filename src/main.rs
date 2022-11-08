extern crate sdl2;

mod tools;
mod config;
mod colors_rect;
mod colors_line;

use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::colors_rect::ColorsRect;
use crate::colors_line::ColorsLine;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("mycolor", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut colors_rect: ColorsRect = ColorsRect::new(Rect::new(20, 20, 400, 200));
    let mut colors_line: ColorsLine = ColorsLine::new(Rect::new(20, 236, 400, 8));

    let mut to_draw = true;

    'running: loop {
        canvas.set_draw_color(Color::RGB(26, 27, 38));
        canvas.clear();

        if to_draw {
            colors_rect.draw(&mut canvas);
            colors_line.draw(&mut canvas);
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
	if mouse.is_mouse_button_pressed(MouseButton::Left) {
	    colors_rect.update(&mouse);
	    colors_line.update(&mouse);
	    colors_rect.set_hue(colors_line.get_hue());
	}

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
