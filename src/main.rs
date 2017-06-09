use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::process::Command;
use std::collections::HashMap;

use std::io::Cursor;

extern crate nom;
extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use maps::*;

mod datfiles;
mod levelexp;
mod maps;


fn usage() {
    println!("")
}


fn main() {
    read_all_maps();

    let initial_file = "";
    let target_file = "";

    // decrypt(initial_file, target_file);

    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn read_all_maps() {
    let maps_folder = "";
    let maps_folder_content = fs::read_dir(maps_folder).unwrap();
    let mut maps_store: Vec<u8>;
    let mut maps: Vec<maps::Map> = Vec::new();

    for map_filename in maps_folder_content {
        let map_filename_unwrapped = map_filename.unwrap().path();
        let map_filename_str = map_filename_unwrapped.to_str();
        let mut map_buff: Vec<u8> = Vec::new();

        match map_filename_str {
            Some(map_filename_actual) => {
                println!("Name: {}", map_filename_actual);
                let buff = maps::read_map_bytes(map_filename_actual);

                match maps::parse_map(&buff) {
                    Some(map) => {
                        println!(" - Map Width: {}", map.get_width());
                        println!(" - Map Height: {}", map.get_height());

                        maps.push(map);
                    },
                    None => {
                        println!("Failed to read map");
                    }
                }
            },
            None => {
                println!("File not found");
            }
        }
    }

    let mut counter_accessible: u64 = 0;
    let mut counter_inaccessible: u64 = 0;

    for map in maps {
        for x in 1..map.get_width() {
            for y in 1..map.get_height() {
                let x = x as u16;
                let y = y as u16;

                if (map.is_accessible_x_y(&x, &y)) {
                    counter_accessible += 1;
                } else {
                    counter_inaccessible += 1;
                }
            }
        }
    }

    println!("Total accessible cells: {}", counter_accessible);
    println!("Total inaccessible cells: {}", counter_inaccessible);
}

fn decrypt(source: &str, dest: &str) {
    // let cofac_key = datfiles::generate_cofac_key();
    // let bytes_read = read_all_bytes(source);
    let level_file = levelexp::LevelExpFile { filename: String::from(source) };
    let levels = level_file.get_levels();
    let levels_hash = level_file.get_levels_as_map();
    // print_levelup(&levels);
    print_levelup_hash(&levels_hash);
    // let bytes_dec = decrypt_bytes(bytes_read, cofac_key);
    // write_all_bytes(dest, bytes_dec);
}




fn read_all_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buffer = Vec::new();

    let bytes_read = f.read_to_end(&mut buffer).unwrap();

    let bytes_read_mb: f64 = (bytes_read as f64) / 1024.0 / 1024.0;

    buffer
}

fn write_all_bytes(filename: &str, bytes: Vec<u8>) {
    let mut f = File::create(filename).unwrap();

    f.write_all(&bytes);
}

fn print_levelup(levels: &Vec<levelexp::Level>) {
	for b in 0..levels.len() {
		let ref current_level = levels[b];

		println!("Level: {}, EXP: {}", current_level.level, current_level.experience);
	}
}

fn print_levelup_hash(levels: &HashMap<u8, i32>) {
	for (level, exp) in levels {
		println!("Level: {}, EXP: {}", level, exp);
	}
}