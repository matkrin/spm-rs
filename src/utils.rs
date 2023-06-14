use std::io::{Cursor, Read};
use std::str;

pub trait Bytereading {
    fn skip(&mut self, num_bytes: u64);
    fn read_matrix_type(&mut self) -> String;
    fn read_matrix_string(&mut self) -> String;
    fn read_utf16_string(&mut self, length: usize) -> String;
    fn read_string(&mut self, length: usize) -> String;
    fn read_magic_header(&mut self) -> String;
    fn read_i8_le(&mut self) -> i8;
    fn read_u8_le(&mut self) -> u8;
    fn read_i16_le(&mut self) -> i16;
    fn read_u16_le(&mut self) -> u16;
    fn read_i32_le(&mut self) -> i32;
    fn read_u32_le(&mut self) -> u32;
    fn read_u64_le(&mut self) -> u64;
    fn read_f32_le(&mut self) -> f32;
    fn read_f64_le(&mut self) -> f64;
}

impl Bytereading for Cursor<&[u8]> {
    fn skip(&mut self, num_bytes: u64) {
        self.set_position(self.position() + num_bytes);
    }

    fn read_matrix_type(&mut self) -> String {
        self.read_string(4).chars().rev().collect()
    }

    fn read_matrix_string(&mut self) -> String {
        let string_length = self.read_u32_le();
        self.read_utf16_string(string_length as usize)
    }

    fn read_utf16_string(&mut self, length: usize) -> String {
        let mut buffer = vec![0; length * 2];
        self.read_exact(&mut buffer).expect("to read");
        read_utf16_bytes(&buffer)
    }

    fn read_string(&mut self, length: usize) -> String {
        let mut buffer = vec![0; length];
        self.read_exact(&mut buffer).expect("to read");
        read_str(&buffer).to_owned()
    }

    fn read_magic_header(&mut self) -> String {
        self.read_string(12)
    }

    fn read_i8_le(&mut self) -> i8 {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer).unwrap();
        read_i8_le_bytes(&buffer)
    }

    fn read_u8_le(&mut self) -> u8 {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer).unwrap();
        read_u8_le_bytes(&buffer)
    }

    fn read_i16_le(&mut self) -> i16 {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer).unwrap();
        read_i16_le_bytes(&buffer)
    }

    fn read_u16_le(&mut self) -> u16 {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer).unwrap();
        read_u16_le_bytes(&buffer)
    }

    fn read_i32_le(&mut self) -> i32 {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer).unwrap();
        read_i32_le_bytes(&buffer)
    }

    fn read_u32_le(&mut self) -> u32 {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer).unwrap();
        read_u32_le_bytes(&buffer)
    }

    fn read_u64_le(&mut self) -> u64 {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer).unwrap();
        read_u64_le_bytes(&buffer)
    }

    fn read_f32_le(&mut self) -> f32 {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer).unwrap();
        read_f32_le_bytes(&buffer)
    }

    fn read_f64_le(&mut self) -> f64 {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer).unwrap();
        read_f64_le_bytes(&buffer)
    }
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

// i8
fn read_i8_le_bytes(buffer: &[u8]) -> i8 {
    i8::from_le_bytes(buffer[..1].try_into().unwrap())
}

// u8
fn read_u8_le_bytes(buffer: &[u8]) -> u8 {
    u8::from_le_bytes(buffer[..1].try_into().unwrap())
}

// i16
pub fn read_i16_le_bytes(buffer: &[u8]) -> i16 {
    i16::from_le_bytes(buffer[..2].try_into().unwrap())
}

// u16
fn read_u16_le_bytes(buffer: &[u8]) -> u16 {
    u16::from_le_bytes(buffer[..2].try_into().unwrap())
}

// i32
fn read_i32_le_bytes(buffer: &[u8]) -> i32 {
    i32::from_le_bytes(buffer[..4].try_into().unwrap())
}

// u32
fn read_u32_le_bytes(buffer: &[u8]) -> u32 {
    u32::from_le_bytes(buffer[..4].try_into().unwrap())
}

// u64
fn read_u64_le_bytes(buffer: &[u8]) -> u64 {
    u64::from_le_bytes(buffer[..8].try_into().unwrap())
}

// f32
fn read_f32_le_bytes(buffer: &[u8]) -> f32 {
    f32::from_le_bytes(buffer[..4].try_into().unwrap())
}

// f64
fn read_f64_le_bytes(buffer: &[u8]) -> f64 {
    f64::from_le_bytes(buffer[..8].try_into().unwrap())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_i8_le_bytes() {
        let n: i8 = 42;
        let bytes = n.to_le_bytes();
        assert_eq!(read_i8_le_bytes(&bytes), n);
    }

    #[test]
    fn test_read_i8_le() {
        let (a, b, c): (i8, i8, i8) = (10, 20, 30);
        let mut buffer = a.to_le_bytes().to_vec();
        buffer.append(&mut b.to_le_bytes().to_vec());
        buffer.append(&mut c.to_le_bytes().to_vec());
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_i8_le(), a);
        assert_eq!(cursor.read_i8_le(), b);
        assert_eq!(cursor.read_i8_le(), c);
    }

    #[test]
    fn test_read_u8_le_bytes() {
        let n: u8 = 42;
        let bytes = n.to_le_bytes();
        assert_eq!(read_u8_le_bytes(&bytes), n);
    }

    #[test]
    fn test_read_u8_le() {
        let (a, b, c): (u8, u8, u8) = (10, 20, 30);
        let mut buffer = a.to_le_bytes().to_vec();
        buffer.append(&mut b.to_le_bytes().to_vec());
        buffer.append(&mut c.to_le_bytes().to_vec());
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_u8_le(), a);
        assert_eq!(cursor.read_u8_le(), b);
        assert_eq!(cursor.read_u8_le(), c);
    }

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
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_i16_le(), a);
        assert_eq!(cursor.read_i16_le(), b);
        assert_eq!(cursor.read_i16_le(), c);
    }

    #[test]
    fn test_read_u16_le_bytes() {
        let n: u16 = 42;
        let bytes = n.to_le_bytes();
        assert_eq!(read_u16_le_bytes(&bytes), n);
    }

    #[test]
    fn test_read_u16_le() {
        let (a, b, c): (u16, u16, u16) = (10, 20, 30);
        let mut buffer = a.to_le_bytes().to_vec();
        buffer.append(&mut b.to_le_bytes().to_vec());
        buffer.append(&mut c.to_le_bytes().to_vec());
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_u16_le(), a);
        assert_eq!(cursor.read_u16_le(), b);
        assert_eq!(cursor.read_u16_le(), c);
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
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_i32_le(), a);
        assert_eq!(cursor.read_i32_le(), b);
        assert_eq!(cursor.read_i32_le(), c);
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
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_u32_le(), a);
        assert_eq!(cursor.read_u32_le(), b);
        assert_eq!(cursor.read_u32_le(), c);
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
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_u64_le(), a);
        assert_eq!(cursor.read_u64_le(), b);
        assert_eq!(cursor.read_u64_le(), c);
    }

    #[test]
    fn test_read_f32_le_bytes() {
        let n: f32 = 42.0;
        let bytes = n.to_le_bytes();
        assert_eq!(read_f32_le_bytes(&bytes), n);
    }

    #[test]
    fn test_read_f32_le() {
        let (a, b, c): (f32, f32, f32) = (10.0, 20.0, 30.0);
        let mut buffer = a.to_le_bytes().to_vec();
        buffer.append(&mut b.to_le_bytes().to_vec());
        buffer.append(&mut c.to_le_bytes().to_vec());
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_f32_le(), a);
        assert_eq!(cursor.read_f32_le(), b);
        assert_eq!(cursor.read_f32_le(), c);
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
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_f64_le(), a);
        assert_eq!(cursor.read_f64_le(), b);
        assert_eq!(cursor.read_f64_le(), c);
    }

    #[test]
    fn test_read_magic_header() {
        let h = "MAGICXHEADER";
        let buffer = h.as_bytes().to_vec();
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_magic_header(), h);
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
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(cursor.read_string(4), "Test");
    }
}
