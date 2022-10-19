use std::io::{Cursor, Read};
use std::str;

pub fn flip_img_data(img_data: Vec<f64>, xres: u32, yres: u32) -> Vec<f64> {
    let mut flipped: Vec<f64> = Vec::with_capacity((xres * yres) as usize);
    for i in (0..yres).rev() {
        let mut line = img_data[(i * xres) as usize..((i + 1) * xres) as usize].to_owned();
        flipped.append(&mut line);
    }
    flipped
}

pub fn skip(cursor: &mut Cursor<&Vec<u8>>, num_bytes: u64) {
    cursor.set_position(cursor.position() + num_bytes);
}

pub fn read_matrix_type(cursor: &mut Cursor<&Vec<u8>>) -> String {
    read_string(cursor, 4).chars().rev().collect()
}

pub fn read_matrix_string(cursor: &mut Cursor<&Vec<u8>>) -> String {
    let string_length = read_u32_le(cursor);
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

fn read_str(buffer: &[u8]) -> &str {
    str::from_utf8(buffer).expect("to read_str")
}

pub fn read_string(cursor: &mut Cursor<&Vec<u8>>, length: usize) -> String {
    let mut buffer = vec![0; length];
    cursor.read_exact(&mut buffer).expect("to read");
    read_str(&buffer).to_owned()
}

pub fn read_magic_header(cursor: &mut Cursor<&Vec<u8>>) -> String {
    read_string(cursor, 12)
}

// i16
pub fn read_i16_le_bytes(buffer: &[u8]) -> i16 {
    i16::from_le_bytes(buffer[..2].try_into().unwrap())
}

pub fn read_i16_le(cursor: &mut Cursor<&Vec<u8>>) -> i16 {
    let mut buffer = [0; 2];
    cursor.read_exact(&mut buffer).unwrap();
    read_i16_le_bytes(&buffer)
}

// i32
fn read_i32_le_bytes(buffer: &[u8]) -> i32 {
    i32::from_le_bytes(buffer[..4].try_into().unwrap())
}

pub fn read_i32_le(cursor: &mut Cursor<&Vec<u8>>) -> i32 {
    let mut buffer = [0; 4];
    cursor.read_exact(&mut buffer).unwrap();
    read_i32_le_bytes(&buffer)
}

fn read_u32_le_bytes(buffer: &[u8]) -> u32 {
    u32::from_le_bytes(buffer[..4].try_into().unwrap())
}

pub fn read_u32_le(cursor: &mut Cursor<&Vec<u8>>) -> u32 {
    let mut buffer = [0; 4];
    cursor.read_exact(&mut buffer).unwrap();
    read_u32_le_bytes(&buffer)
}

fn read_u64_le_bytes(buffer: &[u8]) -> u64 {
    u64::from_le_bytes(buffer[..8].try_into().unwrap())
}

pub fn read_u64_le(cursor: &mut Cursor<&Vec<u8>>) -> u64 {
    let mut buffer = [0; 8];
    cursor.read_exact(&mut buffer).unwrap();
    read_u64_le_bytes(&buffer)
}

fn read_f64_le_bytes(buffer: &[u8]) -> f64 {
    f64::from_le_bytes(buffer[..8].try_into().unwrap())
}

pub fn read_f64_le(cursor: &mut Cursor<&Vec<u8>>) -> f64 {
    let mut buffer = [0; 8];
    cursor.read_exact(&mut buffer).unwrap();
    read_f64_le_bytes(&buffer)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_i16_le_bytes() {
        let n: i16 = 42;
        let bytes = n.to_le_bytes();
        assert_eq!(read_i16_le_bytes(&bytes), n);
    }

    #[test]
    fn test_read_i16_le() {
        let (a, b, c): (i16, i16, i16) = (10, 20, 30);
        let mut buffer = a.to_le_bytes().to_vec();
        buffer.append(&mut b.to_le_bytes().to_vec());
        buffer.append(&mut c.to_le_bytes().to_vec());
        let mut cursor = Cursor::new(&buffer);
        assert_eq!(read_i16_le(&mut cursor), a);
        assert_eq!(read_i16_le(&mut cursor), b);
        assert_eq!(read_i16_le(&mut cursor), c);
    }

    #[test]
    fn test_read_i32_le_bytes() {
        let n: i32 = 42;
        let bytes = n.to_le_bytes();
        assert_eq!(read_i32_le_bytes(&bytes), n);
    }

    #[test]
    fn test_read_i32_le() {
        let (a, b, c): (i32, i32, i32) = (10, 20, 30);
        let mut buffer = a.to_le_bytes().to_vec();
        buffer.append(&mut b.to_le_bytes().to_vec());
        buffer.append(&mut c.to_le_bytes().to_vec());
        let mut cursor = Cursor::new(&buffer);
        assert_eq!(read_i32_le(&mut cursor), a);
        assert_eq!(read_i32_le(&mut cursor), b);
        assert_eq!(read_i32_le(&mut cursor), c);
    }

    #[test]
    fn test_read_u32_le_bytes() {
        let n: u32 = 42;
        let bytes = n.to_le_bytes();
        assert_eq!(read_u32_le_bytes(&bytes), n);
    }

    #[test]
    fn test_read_u32_le() {
        let (a, b, c): (u32, u32, u32) = (10, 20, 30);
        let mut buffer = a.to_le_bytes().to_vec();
        buffer.append(&mut b.to_le_bytes().to_vec());
        buffer.append(&mut c.to_le_bytes().to_vec());
        let mut cursor = Cursor::new(&buffer);
        assert_eq!(read_u32_le(&mut cursor), a);
        assert_eq!(read_u32_le(&mut cursor), b);
        assert_eq!(read_u32_le(&mut cursor), c);
    }

    #[test]
    fn test_read_u64_le_bytes() {
        let n: u64 = 42;
        let bytes = n.to_le_bytes();
        assert_eq!(read_u64_le_bytes(&bytes), n);
    }

    #[test]
    fn test_read_u64_le() {
        let (a, b, c): (u64, u64, u64) = (10, 20, 30);
        let mut buffer = a.to_le_bytes().to_vec();
        buffer.append(&mut b.to_le_bytes().to_vec());
        buffer.append(&mut c.to_le_bytes().to_vec());
        let mut cursor = Cursor::new(&buffer);
        assert_eq!(read_u64_le(&mut cursor), a);
        assert_eq!(read_u64_le(&mut cursor), b);
        assert_eq!(read_u64_le(&mut cursor), c);
    }

    #[test]
    fn test_read_f64_le_bytes() {
        let n: f64 = 42.0;
        let bytes = n.to_le_bytes();
        assert_eq!(read_f64_le_bytes(&bytes), n);
    }

    #[test]
    fn test_read_f64_le() {
        let (a, b, c): (f64, f64, f64) = (10.0, 20.0, 30.0);
        let mut buffer = a.to_le_bytes().to_vec();
        buffer.append(&mut b.to_le_bytes().to_vec());
        buffer.append(&mut c.to_le_bytes().to_vec());
        let mut cursor = Cursor::new(&buffer);
        assert_eq!(read_f64_le(&mut cursor), a);
        assert_eq!(read_f64_le(&mut cursor), b);
        assert_eq!(read_f64_le(&mut cursor), c);
    }

    #[test]
    fn test_read_magic_header() {
        let h = "MAGICXHEADER";
        let buffer = h.as_bytes().to_vec();
        let mut cursor = Cursor::new(&buffer);
        assert_eq!(read_magic_header(&mut cursor), h);
    }

    #[test]
    fn test_read_str() {
        let s = "Test string";
        let sb = s.as_bytes();
        assert_eq!(read_str(sb), s);
    }

    #[test]
    pub fn test_read_string() {
        let s = "Test string";
        let buffer = s.as_bytes().to_vec();
        let mut cursor = Cursor::new(&buffer);
        assert_eq!(read_string(&mut cursor, 4), "Test");
    }
}
