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

#[macro_use]
extern crate nom;
extern crate clap;
extern crate serde_json;
extern crate byteorder;
extern crate regex;
extern crate ini;

#[macro_use]
extern crate serde_derive;

use clap::{Arg, App, SubCommand, ArgMatches};

#[macro_use]
mod datfiles;
mod levelexp;
mod itemtype;
mod magictype;
mod monster;


fn get_app_usage<'a>() -> ArgMatches<'a> {
    App::new("CO V5165 Toolkit")
        .version("0.4.0")
        .author("Tarouka <tarouka@openmailbox.org>")

        .subcommand(SubCommand::with_name("decrypt_dat")
            .about("Decrypts a standard CO dat file (itemtype.dat, Monster.dat or MagicType.dat).")
            .arg(Arg::with_name("SRC_FILENAME").help("Source filename").required(true))
            .arg(Arg::with_name("DST_FILENAME").help("Destination filename").required(true))
        )

        .subcommand(SubCommand::with_name("encrypt_dat")
            .about("Encrypts a standard CO dat file (itemtype.dat, Monster.dat or MagicType.dat).")
            .arg(Arg::with_name("SRC_FILENAME").help("Source filename").required(true))
            .arg(Arg::with_name("DST_FILENAME").help("Destination filename").required(true))
        )

        .subcommand(SubCommand::with_name("itemtype")
            .about("Encodes or decodes an itemtype file to a parseable format.")
            
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

        .subcommand(SubCommand::with_name("magictype")
            .about("Encodes or decodes an magictype file to a parseable format.")
            
            .subcommand(SubCommand::with_name("decode")
                .about("Decodes a magictype.dat file to a more workable format.")

                .arg(Arg::with_name("output-format").long("output-format").short("o").takes_value(true).possible_values(&["json"]).help("Assumes JSON by default"))
                .arg(Arg::with_name("decrypt").long("decrypt").short("d").help("The file will be decrypted before processing"))
                .arg(Arg::with_name("FROM_FILE").help("Source filename").required(true))
                .arg(Arg::with_name("TO_FILE").help("Destination filename").required(true))
            )

            .subcommand(SubCommand::with_name("encode")
                .about("Encodes back a magictype.dat decoded format.")

                .arg(Arg::with_name("FROM_FILE").help("Source filename").required(true))
                .arg(Arg::with_name("TO_FILE").help("Destination filename").required(true))
                .arg(Arg::with_name("output-format").long("output-format").short("o").takes_value(true).possible_values(&["json"]).help("Assumes the format from the extension by default"))
                .arg(Arg::with_name("encrypt").long("encrypt").short("e").help("The file will be encrypted"))
            )
        )

        .subcommand(SubCommand::with_name("monster")
            .about("Encodes or decodes an monster file to a parseable format.")
            
            .subcommand(SubCommand::with_name("decode")
                .about("Decodes a Monster.dat file to a more workable format.")

                .arg(Arg::with_name("output-format").long("output-format").short("o").takes_value(true).possible_values(&["json"]).help("Assumes JSON by default"))
                .arg(Arg::with_name("decrypt").long("decrypt").short("d").help("The file will be decrypted before processing"))
                .arg(Arg::with_name("FROM_FILE").help("Source filename").required(true))
                .arg(Arg::with_name("TO_FILE").help("Destination filename").required(true))
            )

            .subcommand(SubCommand::with_name("encode")
                .about("Encodes back a Monster.dat decoded format.")

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
        commands::exec_decrypt_dat(&matches);
    }

    else if let Some(matches) = matches.subcommand_matches("encrypt_dat") {
        commands::exec_encrypt_dat(&matches);
    }

    else if let Some(matches) = matches.subcommand_matches("itemtype") {
        if let Some(matches) = matches.subcommand_matches("decode") {
            commands::exec_itemtype_decode(&matches);
        }

        if let Some(matches) = matches.subcommand_matches("encode") {
            commands::exec_itemtype_encode(&matches);
        }
    }

    else if let Some(matches) = matches.subcommand_matches("magictype") {
        if let Some(matches) = matches.subcommand_matches("decode") {
            commands::exec_magictype_decode(&matches);
        }

        if let Some(matches) = matches.subcommand_matches("encode") {
            commands::exec_magictype_encode(&matches);
        }
    }

    else if let Some(matches) = matches.subcommand_matches("monster") {
        if let Some(matches) = matches.subcommand_matches("decode") {
            commands::exec_monster_decode(&matches);
        }

        if let Some(matches) = matches.subcommand_matches("encode") {
            commands::exec_monster_encode(&matches);
        }
    }

    else {
        println!("{}", matches.usage());
    }
}

pub fn decrypt_cofac_dat(source: &str) -> Vec<u8> {
    let cofac_key = datfiles::generate_cofac_key();
    println!("Generated COFAC key successfully!");

    let bytes_read = read_all_bytes(source);
    println!("File successfully read.");

    let bytes_dec = datfiles::decrypt_bytes(&bytes_read, &cofac_key);
    println!("Decryption complete.");

    bytes_dec
}

pub fn encrypt_cofac_bytes(source_bytes: Vec<u8>) -> Vec<u8> {
    let cofac_key = datfiles::generate_cofac_key();
    println!("Generated COFAC key successfully!");

    let bytes_enc = datfiles::encrypt_bytes(&source_bytes, &cofac_key);
    println!("Encryption complete.");

    bytes_enc
}

pub fn encrypt_cofac_dat(source: &str) -> Vec<u8> {
    let cofac_key = datfiles::generate_cofac_key();
    println!("Generated COFAC key successfully!");

    let bytes_read = read_all_bytes(source);
    println!("File successfully read.");

    let bytes_enc = datfiles::encrypt_bytes(&bytes_read, &cofac_key);
    println!("Encryption complete.");

    bytes_enc
}

pub fn read_all_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buffer = Vec::new();

    let bytes_read = f.read_to_end(&mut buffer).unwrap();

    buffer
}

pub fn write_all_bytes(filename: &str, bytes: Vec<u8>) {
    let mut f = File::create(filename).unwrap();

    f.write_all(&bytes);
}

mod commands {
    use super::*;
    use datfiles::parser::ParserSerializable;
    use ini::Ini;
    use monster::parser::GetKeyError;


    struct DecoderArgs<'a> {
        src_filename: &'a str,
        dst_filename: &'a str,
        format: &'a str,
        decrypt: bool
    }

    struct EncoderArgs<'a> {
        src_filename: &'a str,
        dst_filename: &'a str,
        format: &'a str,
        encrypt: bool
    }

    fn get_decoder_args<'a>(matches: &'a ArgMatches) -> DecoderArgs<'a> {
        DecoderArgs {
            src_filename: matches.value_of("FROM_FILE").unwrap(),
            dst_filename: matches.value_of("TO_FILE").unwrap(),
            format: matches.value_of("format").unwrap_or("json"),
            decrypt: matches.is_present("decrypt")
        }
    }

    fn get_encoder_args<'a>(matches: &'a ArgMatches) -> EncoderArgs<'a> {
        EncoderArgs {
            src_filename: matches.value_of("FROM_FILE").unwrap(),
            dst_filename: matches.value_of("TO_FILE").unwrap(),
            format: matches.value_of("format").unwrap_or("json"),
            encrypt: matches.is_present("encrypt")
        }
    }

    pub fn exec_decrypt_dat<'a>(matches: &'a ArgMatches) {
        let src_filename = matches.value_of("SRC_FILENAME").unwrap();
        let dst_filename = matches.value_of("DST_FILENAME").unwrap();

        let bytes_dec = decrypt_cofac_dat(src_filename);

        write_all_bytes(dst_filename, bytes_dec);
        println!("Wrote decrypted file to {} successfully.", dst_filename);
    }

    pub fn exec_encrypt_dat<'a>(matches: &'a ArgMatches) {
        let src_filename = matches.value_of("SRC_FILENAME").unwrap();
        let dst_filename = matches.value_of("DST_FILENAME").unwrap();

        let bytes_enc = encrypt_cofac_dat(src_filename);

        write_all_bytes(dst_filename, bytes_enc);
        println!("Wrote encrypted file to {} successfully.", dst_filename);
    }

    pub fn exec_itemtype_decode<'a>(matches: &'a ArgMatches) {
        let args = get_decoder_args(matches);

        let src_bytes = if args.decrypt { decrypt_cofac_dat(args.src_filename) } else { read_all_bytes(&args.src_filename) };
        let parsed_file = itemtype::parser::parse_item_type(&src_bytes);

        let mut bytes_to_write: Vec<u8> = Vec::new();

        match parsed_file {
            Some(item_type) => {
                if args.format == "json" {
                    bytes_to_write = itemtype::encoder::decode_item_type_to_json(&item_type);
                }
            },

            None => {
                panic!("An error occured while parsing the file.");
            }
        }

        write_all_bytes(&args.dst_filename, bytes_to_write);
    }

    pub fn exec_itemtype_encode<'a>(matches: &'a ArgMatches) {
        let args = get_encoder_args(matches);

        let src_bytes = read_all_bytes(&args.src_filename);
        let decoded_file = itemtype::encoder::encode_item_type_from_json(src_bytes);

        let mut encoded_bytes: Vec<u8> = Vec::new();

        if args.format == "json" {
            encoded_bytes = decoded_file.serialize_to_string().into_bytes();
        } else {
            panic!("Unsupported format");
        }

        let mut bytes_to_write = if args.encrypt { encrypt_cofac_bytes(encoded_bytes) } else { encoded_bytes };

        write_all_bytes(&args.dst_filename, bytes_to_write);
    }

    pub fn exec_magictype_decode<'a>(matches: &'a ArgMatches) {
        let args = get_decoder_args(matches);

        let src_bytes = if args.decrypt { decrypt_cofac_dat(args.src_filename) } else { read_all_bytes(&args.src_filename) };
        let parsed_file = magictype::parser::parse_magic_type(&src_bytes);

        let mut bytes_to_write: Vec<u8> = Vec::new();

        match parsed_file {
            Some(magic_type) => {
                if args.format == "json" {
                    bytes_to_write = magictype::encoder::decode_magic_type_to_json(&magic_type);
                }
            },

            None => {
                panic!("An error occured while parsing the file.");
            }
        }

        write_all_bytes(&args.dst_filename, bytes_to_write);
    }

    pub fn exec_magictype_encode<'a>(matches: &'a ArgMatches) {
        let args = get_encoder_args(matches);

        let src_bytes = read_all_bytes(&args.src_filename);
        let decoded_file = magictype::encoder::encode_magic_type_from_json(src_bytes);

        let mut encoded_bytes: Vec<u8> = Vec::new();

        if args.format == "json" {
            encoded_bytes = decoded_file.serialize_to_string().into_bytes();
        } else {
            panic!("Unsupported format");
        }

        let mut bytes_to_write = if args.encrypt { encrypt_cofac_bytes(encoded_bytes) } else { encoded_bytes };

        write_all_bytes(&args.dst_filename, bytes_to_write);
    }

    pub fn exec_monster_decode<'a>(matches: &'a ArgMatches) {
        let args = get_decoder_args(matches);

        let src_bytes = if args.decrypt { decrypt_cofac_dat(args.src_filename) } else { read_all_bytes(&args.src_filename) };
        let str_val = &String::from_utf8_lossy(&src_bytes);
        println!("{}", str_val);
        let ini = Ini::load_from_str(str_val).unwrap();
        let parsed_file = monster::parser::get_all_monster_entries(ini);

        let mut bytes_to_write: Vec<u8> = Vec::new();

        match parsed_file {
            Ok(monster_entries) => {
                if args.format == "json" {
                    bytes_to_write = monster::encoder::decode_monster_to_json(&monster_entries);
                }
            },

            Err(GetKeyError::KeyNotFound(err)) => {
                println!("Key not found: {}", err);
                panic!("");
            },

            Err(GetKeyError::FailedToParse(err)) => {
                println!("Failed to parse: {}", err);
                panic!("");
            }
        }

        write_all_bytes(&args.dst_filename, bytes_to_write);
    }

    pub fn exec_monster_encode<'a>(matches: &'a ArgMatches) {
        let args = get_encoder_args(matches);

        let src_bytes = read_all_bytes(&args.src_filename);
        let decoded_file = monster::encoder::encode_monster_from_json(src_bytes);

        let mut encoded_bytes: Vec<u8> = Vec::new();

        if args.format == "json" {
            let str_bytes: Vec<String> = decoded_file.iter().map(|bytes| bytes.serialize_to_string() + "\n\n").collect();
            encoded_bytes = str_bytes.concat().into_bytes();
        } else {
            panic!("Unsupported format");
        }

        let mut bytes_to_write = if args.encrypt { encrypt_cofac_bytes(encoded_bytes) } else { encoded_bytes };

        write_all_bytes(&args.dst_filename, bytes_to_write);
    }
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