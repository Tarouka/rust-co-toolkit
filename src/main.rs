#![recursion_limit="128"]

use std::io;
use std::io::*;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::process::Command;
use std::collections::HashMap;
use std::str;

use std::io::Cursor;
use std::env;

extern crate nom;
extern crate byteorder;
extern crate clap;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use clap::{Arg, App, SubCommand, ArgMatches};

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use nom::*;
use itemtype::parser::ParserSerializable;

mod datfiles;
mod levelexp;
mod itemtype;


fn get_app_usage<'a>() -> ArgMatches<'a> {
    App::new("CO V5165 Toolkit")
        .version("0.2.0")
        .author("Tarouka <tarouka@openmailbox.org>")

        .subcommand(SubCommand::with_name("decrypt_dat")
            .about("Decrypts a standard CO dat file (itemtype.dat, Monster.dat or MagicType.dat).")
            .arg(Arg::with_name("SRC_FILENAME").help("Source filename").required(true))
            .arg(Arg::with_name("DST_FILENAME").help("Destination filename").required(true))
        )

        .subcommand(SubCommand::with_name("itemtype")
            .subcommand(SubCommand::with_name("decode")
                .about("Decodes an itemtype.dat file to a more workable format.")

                .arg(Arg::with_name("output-format").long("output-format").short("o").takes_value(true).possible_values(&["json"]).help("Assumes JSON by default"))
                .arg(Arg::with_name("decrypt").long("decrypt").short("d").help("The file will be decrypted before processing"))
                .arg(Arg::with_name("FROM_FILE").help("Source filename").required(true))
                .arg(Arg::with_name("TO_FILE").help("Destination filename").required(true))
            )

            .subcommand(SubCommand::with_name("encode")
                .about("Encodes back an itemtype.dat decoded format.")

                .arg(Arg::with_name("FROM_FILE").help("Source filename").required(true))
                .arg(Arg::with_name("TO_FILE").help("Destination filename").required(true))
                .arg(Arg::with_name("output-format").long("output-format").short("o").takes_value(true).possible_values(&["json"]).help("Assumes the format from the extension by default"))
                .arg(Arg::with_name("encrypt").long("encrypt").short("e").help("The file will be encrypted"))
            )

        )
        .get_matches()

}


fn main() {
    let matches = get_app_usage();

    if let Some(matches) = matches.subcommand_matches("decrypt_dat") {
        prepare_decrypt_dat(&matches);
    }

    if let Some(matches) = matches.subcommand_matches("itemtype") {
        if let Some(matches) = matches.subcommand_matches("decode") {
            prepare_itemtype_decode(&matches);
        }

        if let Some(matches) = matches.subcommand_matches("encode") {
            prepare_itemtype_encode(&matches);
        }
    }
}

fn prepare_decrypt_dat<'a>(matches: &'a ArgMatches) {
    let src_filename = matches.value_of("SRC_FILENAME").unwrap();
    let dst_filename = matches.value_of("DST_FILENAME").unwrap();

    exec_decrypt_dat(&src_filename, &dst_filename);
}

fn exec_decrypt_dat(source: &str, dest: &str) {
    let cofac_key = datfiles::generate_cofac_key();
    println!("Generated COFAC key successfully!");

    let bytes_read = read_all_bytes(source);
    println!("File successfully read.");

    let bytes_dec = datfiles::decrypt_bytes(&bytes_read, &cofac_key);
    println!("Decryption complete.");

    write_all_bytes(dest, bytes_dec);
    println!("Wrote decrypted file to {} successfully.", dest);
}

fn prepare_itemtype_decode<'a>(matches: &'a ArgMatches) {
    let src_filename = matches.value_of("FROM_FILE").unwrap();
    let dst_filename = matches.value_of("TO_FILE").unwrap();
    let format = matches.value_of("format").unwrap_or("json");
    let decrypt = matches.is_present("decrypt");

    let src_bytes = read_all_bytes(&src_filename);
    let parsed_file = itemtype::parser::parse_item_type(&src_bytes);

    let mut decoded_bytes: Vec<u8> = Vec::new();

    match parsed_file {
        Some(item_type) => {
            if format == "json" {
                decoded_bytes = itemtype::encoder::decode_item_type_to_json(&item_type);
            }
        },

        None => {
            println!("error parsing file");
        }
    }

    let mut bytes_to_write = decoded_bytes;

    if decrypt {
        let cofac_key = datfiles::generate_cofac_key();
        bytes_to_write = datfiles::decrypt_bytes(&bytes_to_write, &cofac_key);
    }

    write_all_bytes(&dst_filename, bytes_to_write);
}

fn prepare_itemtype_encode<'a>(matches: &'a ArgMatches) {
    let src_filename = matches.value_of("FROM_FILE").unwrap();
    let dst_filename = matches.value_of("TO_FILE").unwrap();
    let format = matches.value_of("format").unwrap_or("json");
    let encrypt = matches.is_present("encrypt");

    let src_bytes = read_all_bytes(&src_filename);
    let decoded_file = itemtype::encoder::encode_item_type_from_json(src_bytes);

    let mut encoded_bytes: Vec<u8> = Vec::new();

    if format == "json" {
        encoded_bytes = decoded_file.serialize_to_string().into_bytes();
    }

    let mut bytes_to_write = encoded_bytes;

    if encrypt {
        let cofac_key = datfiles::generate_cofac_key();
        bytes_to_write = datfiles::encrypt_bytes(&bytes_to_write, &cofac_key);
    }

    write_all_bytes(&dst_filename, bytes_to_write);
}

fn read_all_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buffer = Vec::new();

    let bytes_read = f.read_to_end(&mut buffer).unwrap();

    buffer
}

fn write_all_bytes(filename: &str, bytes: Vec<u8>) {
    let mut f = File::create(filename).unwrap();

    f.write_all(&bytes);
}




/*fn read_all_maps() {
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
}*/