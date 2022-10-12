use std::collections::HashMap; use std::fs::read;
use std::io::{Cursor, Read};
use std::str;

pub fn read_omicron_matrix_scanfile(filename: &str) {
    let bytes = read(filename).unwrap();
    let mut cursor = Cursor::new(&bytes);
    let magic_header = read_magic_header(&mut cursor);
    assert_eq!(magic_header, "ONTMATRX0101");

    let file_length = bytes.len();
    let mut position = 0;
    // while position < file_length as u64 {
    //     let i = read_ident_block(&mut cursor);
    //     println!("i: {:?}", i);
    //     position = cursor.position();
    // }
}

fn read_str(buffer: &[u8]) -> &str {
    str::from_utf8(buffer).expect("to read_str")
}


fn read_magic_header(cursor: &mut Cursor<&Vec<u8>>) -> String {
    let mut buffer = [0; 12];
    cursor.read_exact(&mut buffer).unwrap();
    read_str(&buffer).to_owned()
}
