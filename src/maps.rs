use nom::*;

use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::collections::HashMap;


pub struct Map {
	header: MapHeader,
	cells: Box<Vec<MapCell>>
}

impl Map {
	pub fn get_version(&self) -> u32 {
		self.header.version
	}

	pub fn get_width(&self) -> u32 {
		self.header.width
	}

	pub fn get_height(&self) -> u32 {
		self.header.height
	}

	pub fn get_idx_for_x_y(&self, x: &u16, y: &u16) -> usize {
		let x = *x - 1;
		let y = *y - 1;

		(y as u32 * self.get_width() + x as u32) as usize
	}

	pub fn get_cell_at_x_y(&self, x: &u16, y: &u16) -> &MapCell {
		let idx = self.get_idx_for_x_y(x, y);
		let cell: &MapCell = &self.cells[idx];

		cell
	}

	pub fn is_accessible_x_y(&self, x: &u16, y: &u16) -> bool {
		let cell = self.get_cell_at_x_y(x, y);

		cell.mask != 0
	}

	pub fn is_accessible_point(&self, point: &MapCoordinate2D) -> bool {
		self.is_accessible_x_y(&point.get_x(), &point.get_y())
	}
}

#[derive(Debug)]
pub struct MapCell {
	mask: u16,
	terrain: u16,
	altitude: i16
}

#[derive(Debug)]
pub struct MapHeader {
	pub version: u32,
	pub data: u32,
	path: Box<Vec<u8>>,
	pub width: u32,
	pub height: u32
}

#[derive(Debug)]
pub struct MapCoordinate2D {
	x: u16,
	y: u16
}

impl MapCoordinate2D {
	pub fn get_x(&self) -> u16 {
		self.x
	}

	pub fn get_y(&self) -> u16 {
		self.y
	}
}

#[derive(Debug)]
pub struct MapRowChecksum {
	checksum: u32
}


#[cfg(test)]
mod tests {
	use super::*;

	fn get_map_struct_with_wh(width: u32, height: u32) -> Map {
		let cells: Vec<MapCell> = Vec::new();
    	let map_path: Vec<u8> = Vec::new();
    	let map = Map { 
    		header: MapHeader {
    			version: 0,
    			data: 0,
    			path: Box::new(map_path),
    			width: width,
    			height: height
    		},
    		cells: Box::new(cells)
    	};

    	map
	}

	fn assert_coords_for_whxy_ok(width: u32, height: u32, x: u16, y: u16, expected: usize) {
		let map = get_map_struct_with_wh(width, height);
    	let result = map.get_idx_for_x_y(&x, &y);

    	assert_eq!(expected, result);
	}

	#[test]
    fn get_idx_valid_first_row_returns_right_values() {
    	assert_coords_for_whxy_ok(320, 140, 319, 1, 318);
    	assert_coords_for_whxy_ok(34100, 1400, 891, 1, 890);
    	assert_coords_for_whxy_ok(1921, 1000, 1920, 1, 1919);
    	assert_coords_for_whxy_ok(1921, 1000, 1921, 1, 1920);
    	assert_coords_for_whxy_ok(1921, 1, 1921, 1, 1920);
    }

    #[test]
    fn get_idx_valid_first_col_returns_right_values() {
    	assert_coords_for_whxy_ok(1400, 8900, 1, 781, 1092000);
    	assert_coords_for_whxy_ok(18000, 8900, 1, 8081, 145440000);
    	assert_coords_for_whxy_ok(1111, 8900, 1, 7682, 8533591);
    	assert_coords_for_whxy_ok(1111, 8900, 1, 1, 0);
    	assert_coords_for_whxy_ok(1111, 8900, 1, 8900, 9886789);
    }
}

mod parser {
	use nom::*;
	use super::*;

	pub fn map_header(input: &[u8]) -> IResult<&[u8], MapHeader> {
		do_parse!(input,
			version: 		le_u32								>>
			data:			le_u32								>>
			path:			take!(260)							>>
			width:			le_u32								>>
			height:			le_u32								>>

			(
				MapHeader {
					version: version,
					data: data,
					path: Box::new(path.to_vec()),
					width: width,
					height: height
				}
			)
		)
	}

	pub fn map_cell(input: &[u8]) -> IResult<&[u8], MapCell> {
		do_parse!(input,
			mask:			le_u16	>>
			terrain:		le_u16	>>
			altitude:		le_i16	>>

			(
				MapCell {
					mask: mask,
					terrain: terrain,
					altitude: altitude
				}
			)
		)
	}

	pub fn map_row_checksum(input: &[u8]) -> IResult<&[u8], MapRowChecksum> {
		do_parse!(input,
			checksum:		le_u32	>>

			(
				MapRowChecksum {
					checksum: checksum
				}
			)
		)
	}

	#[cfg(test)]
	mod tests {
		use nom::*;
		use super::*;

		#[test]
		fn test_parse_valid_map_cell() {
			let map_cell_bytes = vec![01, 00, 02, 00, 03, 00];

			match map_cell(&map_cell_bytes) {
				IResult::Done(_, cell) => {
					assert_eq!(1, cell.mask);
					assert_eq!(2, cell.terrain);
					assert_eq!(3, cell.altitude);
				},
				e => {
					panic!("An error has occured while reading map cell");
				}
			}
		}

		#[test]
		fn test_parse_invalid_map_cell() {
			let map_cell_bytes = vec![01, 00, 02, 00, 03];

			match map_cell(&map_cell_bytes) {
				IResult::Done(_, cell) => {
					panic!("Cell should be invalid (given 5 bytes, 6 are required).");
				},
				e => {
					assert!(true);
				}
			}
		}
	}
}

pub fn read_map_bytes(filename: &str) -> Vec<u8> {
	let mut f = File::open(filename).unwrap();
    let mut buffer = Vec::new();

    let bytes_read = f.read_to_end(&mut buffer).unwrap();

    let bytes_read_mb: f64 = (bytes_read as f64) / 1024.0 / 1024.0;

    buffer
}

pub fn parse_map<'a>(bytes: &'a [u8]) -> Option<Map> {
	let mut map_cells: Vec<MapCell> = Vec::new();

	println!("{}", bytes.len());
	match parser::map_header(bytes) {
		IResult::Done(_, header) => {
			for y in 0..header.height {
				/** CheckData += (UInt32)((Layer.Mask * (Layer.Terrain + i + 1)) +
                                              ((Layer.Altitude + 2) * (j + 1 + Layer.Terrain)));*/
				let mut check_data = 0;
				for x in 0..header.width {
					let slice_start = 276 + ((y * header.width + x) * 6 + y * 4) as usize;
					// println!("{}", slice_start);
					let processed_bytes = &bytes[slice_start..];

					match parser::map_cell(processed_bytes) {
						IResult::Done(_, cell) => {
							check_data += (cell.mask as i32 * (cell.terrain as i32 + y as i32 + 1) + (cell.altitude as i32 + 2) * (x as i32 + 1 + cell.terrain as i32)) as u32;
							// println!("Mask: {} Terrain: {}, Altitude: {}", cell.mask, cell.terrain, cell.altitude);
							map_cells.push(cell);
						},
						e => {
							return None;
						}
					}
				}

				let slice_start = 276 + ((y + 1) * header.width * 6 + y * 4) as usize;
				let processed_bytes = &bytes[slice_start..];

				match parser::map_row_checksum(processed_bytes) {
					IResult::Done(_, row_checksum) => {
						if (check_data != row_checksum.checksum) {
							panic!("Map has wrong checksum");
						}
					},
					e => {
						return None;
					}
				}
			}

			let map = Map { header: header, cells: Box::new(map_cells) };

			return Some(map);			
		},
		e => {
			return None;
		}
	}
}