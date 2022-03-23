use crate::cartridge::Mirroring;
use registers::addr::AddrRegister;
use registers::control::ControlRegister;

pub mod registers;

pub struct NesPPU {
	pub chr_rom: Vec<u8>,
	pub palette_table: [u8; 32],
	pub vram: [u8; 2048],
	pub oam_data: [u8; 256],
	pub addr: AddrRegister,
	pub ctrl: ControlRegister,

	pub mirroring: Mirroring,

	internal_data_buf: u8,
}

impl NesPPU {
	pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
		NesPPU {
			chr_rom: chr_rom,
			mirroring: mirroring,
			vram: [0; 2048],
			oam_data: [0; 64 * 4],
			palette_table: [0; 32],
		}
	}

	fn write_to_ppu_addr(&mut self, value: u8) {
		self.addr.update(value);
	}

	fn write_to_ctrl(&mut self, value: u8) {
		self.ctrl.update(value);
	}

	fn increment_vram_addr(&mut self) {
		self.addr.increment(self.ctrl.vram_addr_increment());
	}

	fn read_data(&mut self) -> u8 {
		let addr = self.addr.get();
		self.increment_vram_addr();

		match addr {
			0..=0x1fff => {
				let result = self.internal_data_buf;
				self.internal_data_buf = self.chr_rom[addr as usize];
				result
			}

			0x2000..=0x2fff => {
				let result = self.internal_data_buf;
				self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
				result
			}

			0x3000..=0x3eff => panic!("addr space 0x3000..0x3eff is not expected to be used, requested = {} ", addr),

			0x3f00..=0x3fff => {
				self.palette_table[(addr - 0x3f00) as usize]
			}
			_ => panic!("unexpected access to mirrored space {}", addr),
		}
	}
}