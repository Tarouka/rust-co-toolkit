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