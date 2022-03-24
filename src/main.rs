pub mod cpu;
pub mod opcodes;
pub mod bus;
pub mod cartridge;
pub mod trace;
pub mod ppu;
pub mod render;
pub mod joypad;

use std::collections::HashMap;

use cpu::Mem;
use cartridge::Rom;
use cpu::CPU;
use bus::Bus;
use trace::trace;
use render::frame::Frame;
use render::palette;
use ppu::NesPPU;
// use rand::Rng;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

fn main() {
	// init sdl2
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem
		.window("Tile viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().present_vsync().build().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();
	canvas.set_scale(3.0, 3.0).unwrap();

	let creator = canvas.texture_creator();
	let mut texture = creator
		.create_texture_target(PixelFormatEnum::RGB24, 256, 240)
		.unwrap();

	// load the game
	let bytes: Vec<u8> = std::fs::read("tetris.nes").unwrap();
	let rom = Rom::new(&bytes).unwrap();

	let mut frame = Frame::new();

	let mut key_map = HashMap::new();
	key_map.insert(Keycode::Down, joypad::JoypadButton::DOWN);
	key_map.insert(Keycode::Up, joypad::JoypadButton::UP);
	key_map.insert(Keycode::Right, joypad::JoypadButton::RIGHT);
	key_map.insert(Keycode::Left, joypad::JoypadButton::LEFT);
	key_map.insert(Keycode::Space, joypad::JoypadButton::SELECT);
	key_map.insert(Keycode::Return, joypad::JoypadButton::START);
	key_map.insert(Keycode::A, joypad::JoypadButton::BUTTON_A);
	key_map.insert(Keycode::S, joypad::JoypadButton::BUTTON_B);

	// run the game cycle
	let bus = Bus::new(rom, move |ppu: &NesPPU, joypad: &mut joypad::Joypad| {
		render::render(ppu, &mut frame);
		texture.update(None, &frame.data, 256 * 3).unwrap();

		canvas.copy(&texture, None, None).unwrap();

		canvas.present();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => std::process::exit(0),

				Event::KeyDown { keycode, .. } => {
					if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
						joypad.set_button_pressed_status(*key, true);
					}
				}

				Event::KeyUp { keycode, .. } => {
					if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
						joypad.set_button_pressed_status(*key, false);
					}
				}
				_ => { /* do nothing */ }
			}
		}
	});

	let mut cpu = CPU::new(bus);

	cpu.reset();
	cpu.run();
}