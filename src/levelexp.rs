/**
 ini/LevelExp.dat file

   This file is not encrypted as of v5165. Every 4 bytes represent a level EXP written in low-endian. 
   The level repesented is the index of the pointer in a u32 array + 1.
*/

use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::collections::HashMap;

extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};


pub struct Level {
	pub level: u8,
	pub experience: i32
}

pub struct LevelExpFile {
	pub filename: String
}

impl LevelExpFile {
	pub fn get_levels(&self) -> Vec<Level> {
		let mut levels_data: Vec<Level> = Vec::new();
		let levels_buff: Vec<u8> = self.get_file_bytes();
		let levels_count: usize = self.get_levels_count(&levels_buff);

		for b in 0..levels_count {
			let exp: i32 = self.get_exp_from_level(&levels_buff, &b);
			let level: u8 = self.get_level_from_idx(&b);;

			let level_data = Level { level: level, experience: exp };

			levels_data.push(level_data);
		}

		levels_data
	}

	pub fn get_levels_as_map(&self) -> HashMap<u8, i32> {
		let mut levels_data: HashMap<u8, i32> = HashMap::new();
		let levels_buff: Vec<u8> = self.get_file_bytes();
		let levels_count = self.get_levels_count(&levels_buff);

		for b in 0..levels_count {
			let exp: i32 = self.get_exp_from_level(&levels_buff, &b);
			let level: u8 = self.get_level_from_idx(&b);

			levels_data.insert(level, exp);
		}

		levels_data
	}

	fn get_levels_count(&self, levels_data: &Vec<u8>) -> usize {
		levels_data.len() / 4
	}

	fn get_level_from_idx(&self, idx: &usize) -> u8 {
		*idx as u8 + 1
	}

	fn get_exp_from_level(&self, levels: &Vec<u8>, idx: &usize) -> i32 {
		let exp_window = &levels[*idx * 4..*idx * 4 + 4];
		let exp: i32 = 0 - LittleEndian::read_i32(&exp_window);

		exp
	}

	fn get_file_bytes(&self) -> Vec<u8> {
		let mut f = File::open(&self.filename).unwrap();
	    let mut buffer = Vec::new();

	    let bytes_read = f.read_to_end(&mut buffer).unwrap();

	    let bytes_read_mb: f64 = (bytes_read as f64) / 1024.0 / 1024.0;

	    buffer
	}
}
