const COFAC_BASE_SEED: i16 = 0x2537;

pub fn decrypt_bytes(bytes_enc: &Vec<u8>, key: &[u8; 128]) -> Vec<u8> {
    let mut bytes_dec: Vec<u8> = Vec::new();

    for b in 0..bytes_enc.len() {
        let tmp: u16 = (bytes_enc[b] ^ key[b % 128]) as u16;
        let byte: u8 = tmp.wrapping_shl(8 - (b as u32 % 8))
            .wrapping_add(tmp >> (b % 8)) as u8;

        bytes_dec.push(byte);
    }

    bytes_dec
}

pub fn encrypt_bytes(bytes_dec: &Vec<u8>, key: &[u8; 128]) -> Vec<u8> {
    let mut bytes_enc: Vec<u8> = Vec::new();

    for b in 0..bytes_dec.len() {
        let curr_byte: u16 = bytes_dec[b] as u16;
        let tmp: u8 = ((curr_byte.wrapping_shr(8 - (b as u32 % 8))).wrapping_add(curr_byte.wrapping_shl(b as u32 % 8))) as u8;

        let byte: u8 = (tmp ^ key[b % 128]) as u8;

        bytes_enc.push(byte);
    }

    bytes_enc
}

pub fn generate_cofac_key() -> [u8; 128] {
    let mut key: [u8; 128] = [0; 128];
    let mut seed: i32 = COFAC_BASE_SEED as i32; // itemtype.dat & magictype.dat initial seed

    for b in 0..128 {
        seed = seed.wrapping_mul(214013).wrapping_add(2531011);
        key[b] = ((seed >> 16 & 0xFFFF) % 0x100) as u8;
    }

    key
}

#[macro_use]
pub mod parser {
    use nom::{not_line_ending, IResult, ErrorKind};
    use std::str;

    named!(pub parse_str_fragment_crlfeof<&[u8], String>, do_parse!(
        str_val: map_res!(not_line_ending, str::from_utf8) >>
        (String::from(str_val))
    ));

    named!(pub parse_str_fragment<&[u8], String>, do_parse!(
        str_val: map_res!(take_until!(" "), str::from_utf8)      >>
        take!(1)    >>
        (String::from(str_val))
    ));

    macro_rules! parse_str_fragment_to_type (
        ( $i:expr, $type:ty ) => ({
            do_parse!($i,
                str_val: parse_str_fragment             >>
                parsed_val: expr_res!(str_val.parse::<$type>()) >>

                (parsed_val)
            )
        })
    );

    macro_rules! parse_str_fragment_to_type_crlfeof (
        ( $i:expr, $type:ty ) => ({
            do_parse!($i,
                str_val: parse_str_fragment_crlfeof             >>
                (str_val.parse::<$type>().unwrap_or_default())
            )
        })
    );

    #[macro_export]
    macro_rules! serializer_append_field_as_bool {
        ( $item_type: expr, $str: expr, $field: ident ) => ({
            $str.push_str(if $item_type.$field { "1" } else { "0" });
            $str.push_str(" ");
        })
    }

    #[macro_export]
    macro_rules! serializer_append_field {
        ( $item_type: expr, $str: expr, $field: ident ) => ({
            $str.push_str(&$item_type.$field.to_string());
            $str.push_str(" ");
        })
    }

    #[macro_export]
    macro_rules! serializer_append_field_last {
        ( $item_type: expr, $str: expr, $field: ident ) => ({
            $str.push_str(&$item_type.$field.to_string());
        })
    }

    #[macro_export]
    macro_rules! serializer_append_field_last_as_bool {
        ( $item_type: expr, $str: expr, $field: ident ) => ({
            $str.push_str(if $item_type.$field { "1" } else { "0" });
        })
    }


    named!(pub parse_str_fragment_to_bool<&[u8], bool>, do_parse!(
        val: parse_str_fragment_to_type!(u8)      >>
        (val > 0)
    ));

    named!(pub parse_str_fragment_to_u8<&[u8], u8>, parse_str_fragment_to_type!(u8));
    named!(pub parse_str_fragment_to_u16<&[u8], u16>, parse_str_fragment_to_type!(u16));
    named!(pub parse_str_fragment_to_u32<&[u8], u32>, parse_str_fragment_to_type!(u32));
    named!(pub parse_str_fragment_to_u64<&[u8], u64>, parse_str_fragment_to_type!(u64));
    named!(pub parse_str_fragment_to_i32<&[u8], i32>, parse_str_fragment_to_type!(i32));

    named!(pub parse_str_fragment_crlfeof_to_u8<&[u8], u8>, parse_str_fragment_to_type_crlfeof!(u8));
    named!(pub parse_str_fragment_crlfeof_to_u32<&[u8], u32>, parse_str_fragment_to_type_crlfeof!(u32));
    named!(pub parse_str_fragment_crlfeof_to_bool<&[u8], bool>, do_parse!(
        val: parse_str_fragment_to_type_crlfeof!(u8)      >>

        (val == 1)
    ));

    pub fn remove_tildes_from(input: String) -> String {
        let result = str::replace(&input, "~", " ");

        result
    }

    pub fn append_tildes_to(input: String) -> String {
        let result = str::replace(&input, " ", "~");

        result
    }

    pub trait ParserSerializable {
        fn serialize_to_string(&self) -> String;
    }

    pub fn split_bytes_by_lines(bytes: Vec<u8>) -> Vec<Vec<u8>> {
        let mut bytes_split: Vec<Vec<u8>> = Vec::new();
        let mut current_split: Vec<u8> = Vec::new();
        let mut crlf_progress = 0;

        for b in bytes {
            if crlf_progress == 0 && b == 0x0D {
                crlf_progress += 1;
            }

            else if crlf_progress == 1 && b == 0x0A {
                bytes_split.push(current_split);
                current_split = Vec::new();
                crlf_progress = 0;
            }

            else if crlf_progress == 1 && b != 0x0A {
                current_split.push(0x0D);
                current_split.push(0x0A);
                crlf_progress = 0;
            }

            else {
                current_split.push(b);
            }
        }

        bytes_split
    }

    macro_rules! parse_match_to_type(
        ( $value:expr, $type:ty, $idx:expr ) => {
            match $value.parse::<$type>() {
                Ok(s) => {
                    return Result::Ok(s);
                }

                _ => {
                    return Result::Err($idx);
                }
            }
        }
    );

    pub fn parse_match_to_u8(matches: &Vec<String>, idx: usize) -> Result<u8, usize> {
        parse_match_to_type!(matches[idx], u8, idx)
    }

    pub fn parse_match_to_u16(matches: &Vec<String>, idx: usize) -> Result<u16, usize> {
        parse_match_to_type!(matches[idx], u16, idx)
    }

    pub fn parse_match_to_u32(matches: &Vec<String>, idx: usize) -> Result<u32, usize> {
        parse_match_to_type!(matches[idx], u32, idx)
    }

    pub fn parse_match_to_i32(matches: &Vec<String>, idx: usize) -> Result<i32, usize> {
        parse_match_to_type!(matches[idx], i32, idx)
    }

    pub fn parse_match_to_u64(matches: &Vec<String>, idx: usize) -> Result<u64, usize> {
        parse_match_to_type!(matches[idx], u64, idx)
    }

    pub fn parse_match_to_i64(matches: &Vec<String>, idx: usize) -> Result<i64, usize> {
        parse_match_to_type!(matches[idx], i64, idx)
    }

    pub fn parse_match_to_bool(matches: &Vec<String>, idx: usize) -> Result<bool, usize> {
        match parse_match_to_i64(matches, idx) {
            Result::Ok(res) => {
                return Result::Ok(res > 0);
            }

            _ => {
                return Result::Err(idx);
            }
        }
    }
}