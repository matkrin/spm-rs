use std::io::{Cursor, Read};
use std::str;

pub fn skip(cursor: &mut Cursor<&Vec<u8>>, num_bytes: u64) {
    cursor.set_position(cursor.position() + num_bytes);
}

pub fn read_matrix_type(cursor: &mut Cursor<&Vec<u8>>) -> String {
    read_string(cursor, 4).chars().rev().collect()
}

pub fn read_matrix_string(cursor: &mut Cursor<&Vec<u8>>) -> String {
    let string_length = read_u32_le(cursor);
    // println!{"string length: {}", string_length};
    read_utf16_string(cursor, string_length as usize)
}

pub fn read_utf16_string(cursor: &mut Cursor<&Vec<u8>>, length: usize) -> String {
    let mut buffer = vec![0; length * 2];
    cursor.read_exact(&mut buffer).expect("to read");
    read_utf16_bytes(&buffer)
}

pub fn read_utf16_bytes(slice: &[u8]) -> String {
    let iter = (0..(slice.len() / 2)).map(|i| u16::from_le_bytes([slice[2 * i], slice[2 * i + 1]]));

    let result = std::char::decode_utf16(iter)
        .collect::<Result<String, _>>()
        .unwrap();
    result
}

pub fn read_str(buffer: &[u8]) -> &str {
    str::from_utf8(buffer).expect("to read_str")
}

pub fn read_string(cursor: &mut Cursor<&Vec<u8>>, length: usize) -> String {
    let mut buffer = vec![0; length];
    cursor.read_exact(&mut buffer).expect("to read");
    read_str(&buffer).to_owned()
}

pub fn read_magic_header(cursor: &mut Cursor<&Vec<u8>>) -> String {
    let mut buffer = [0; 12];
    cursor.read_exact(&mut buffer).unwrap();
    read_str(&buffer).to_owned()
}

pub fn read_u32_le_bytes(buffer: &[u8]) -> u32 {
    u32::from_le_bytes(buffer[..4].try_into().unwrap())
}

pub fn read_u32_le(cursor: &mut Cursor<&Vec<u8>>) -> u32 {
    let mut buffer = [0; 4];
    cursor.read_exact(&mut buffer).unwrap();
    read_u32_le_bytes(&buffer)
}

pub fn read_u64_le_bytes(buffer: &[u8]) -> u64 {
    u64::from_le_bytes(buffer[..8].try_into().unwrap())
}

pub fn read_u64_le(cursor: &mut Cursor<&Vec<u8>>) -> u64 {
    let mut buffer = [0; 8];
    cursor.read_exact(&mut buffer).unwrap();
    read_u64_le_bytes(&buffer)
}

pub fn read_f64_le_bytes(buffer: &[u8]) -> f64 {
    f64::from_le_bytes(buffer[..8].try_into().unwrap())
}

pub fn read_f64_le(cursor: &mut Cursor<&Vec<u8>>) -> f64 {
    let mut buffer = [0; 8];
    cursor.read_exact(&mut buffer).unwrap();
    read_f64_le_bytes(&buffer)
}
