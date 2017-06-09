use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};


type AccessibilityBitset = u8;

pub struct MapCell {
	accessible: bool,
	x: u16,
	y: u16
}

/*

fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
    where R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    let status = chunk.read_to_end(&mut buf);
    // Do appropriate error handling
    match status {
        Ok(n) => assert_eq!(bytes_to_read as usize, n),
        _ => panic!("Didn't read enough"),
    }
    buf
}

fn main() {
    let input_data = b"hello world";
    let mut reader = BufReader::new(&input_data[..]);

    let first = read_n(&mut reader, 5);
    let _ = read_n(&mut reader, 1);
    let second = read_n(&mut reader, 5);

    println!("{:?}, {:?}", str::from_utf8(&first), str::from_utf8(&second));
}
*/

pub fn get_map_data(filename: &str) -> Map {
	let f = File::open(filename).unwrap();
	let mut f = BufReader::new(f);
	read_n(&mut f, 268);

	let width_b = read_n(&mut f, 4);
	let height_b = read_n(&mut f, 4);

	let width = LittleEndian::read_i32(&width_b);
	let height = LittleEndian::read_i32(&height_b);

	println!("W: {}, H: {}", width, height);

	let mut map = Map { height: height as u16, width: width as u16, accessibility: Box::new(Vec::new()) };
	map.init_accessibility_from_width_height();


	for x in 0..width {
		for y in 0..height {
			let accessible_b = read_n(&mut f, 1);
			let accessible = accessible_b[0] == 1;

			let x = x as u16;
			let y = y as u16;

			map.set_accessibility_for_coords(&x, &y, &accessible);

			read_n(&mut f, 5);
		}

		read_n(&mut f, 4);
	}


	map
}

fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
    where R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    let status = chunk.read_to_end(&mut buf);
    // Do appropriate error handling
    match status {
        Ok(n) => assert_eq!(bytes_to_read as usize, n),
        _ => panic!("Didn't read enough"),
    }
    buf
}

fn get_file_bytes(filename: &str) -> Vec<u8> {
	let mut f = File::open(filename).unwrap();
    let mut buffer = Vec::new();

    let bytes_read = f.read_to_end(&mut buffer).unwrap();

    buffer
}

pub struct Map {
	accessibility: Box<Vec<u8>>,
	height: u16,
	width: u16
}

impl Map {

	pub fn init_accessibility_from_width_height(&mut self) {
		let map_size_idx = (self.width as f64 * self.height as f64 / 8.0).ceil() as usize;
		self.accessibility = Box::new(Vec::with_capacity(map_size_idx));


		for b in 0..map_size_idx {
			self.accessibility.push(0);
		}
	}

	pub fn set_accessibility_for_coords(&self, x: &u16, y: &u16, accessible: &bool) {
		let bit = self.get_accessibility_bitset_bit(x, y);
		let idx = self.get_accessibility_bitset_idx(x, y);

		let mut accessibility_window = &self.accessibility[idx];
		let accessibility_modifier = ((*accessible as u8) >> bit) & 0xFF;

		let accessibility_flag = (accessibility_window & accessibility_modifier);

		//&self.accessibility[idx] = accessibility_window ^ *accessibility_flag
	}

	pub fn get_accessibility_for_coords(&self, x: &u16, y: &u16) -> bool {
		let bit = self.get_accessibility_bitset_bit(x, y);
		let idx = self.get_accessibility_bitset_idx(x, y);

		let accessibility_window = &self.accessibility[idx];

		let accessibility_flag = (accessibility_window & (256 >> bit)) > 1;

		accessibility_flag
	}

	fn get_accessibility_bitset_idx(&self, x: &u16, y: &u16) -> usize {
		((*y as f64 * self.width as f64 + *x as f64) / 8.0).ceil() as usize
	}

	fn get_accessibility_bitset_bit(&self, x: &u16, y: &u16) -> u8 {
		((*y as u64 * self.width as u64 + *x as u64) % 8 + 1) as u8
	}
}
