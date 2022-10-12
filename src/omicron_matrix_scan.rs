use std::fs::read;
use std::io::Cursor;
use std::io::Read;
use std::str;

use crate::utils::{read_magic_header, read_matrix_type, read_u32_le, skip, read_u64_le, read_matrix_string};

pub fn read_omicron_matrix_scanfile(filename: &str) {
    let bytes = read(filename).unwrap();
    let mut cursor = Cursor::new(&bytes);
    let magic_header = read_magic_header(&mut cursor);
    assert_eq!(magic_header, "ONTMATRX0101");

    let file_length = bytes.len();
    println!("file length: {}", file_length);
    let mut position = 0;
    while position < file_length as u64 {
        let i = read_ident_block(&mut cursor);
        // println!("i: {:?}", i);
        position = cursor.position();
    }
    assert_eq!(cursor.position(), file_length as u64);
}

#[derive(Debug)]
enum IdentBlock {
    BKLT(String),
    DESC(String),
    DATA(String),
}

fn read_ident_block(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let ident: String = read_matrix_type(cursor);
    // println!("ident: {}", ident);

    match ident.as_str() {
        "BKLT" => read_bklt(cursor),
        "DESC" => read_desc(cursor),
        "DATA" => read_data(cursor),
        _ => unimplemented!(),
    }
}

fn read_bklt(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    println!("BKLT len: {}", _len);
    let _time = read_u32_le(cursor);
    println!("BKLT time: {}", _time);
    let _unused = read_u32_le(cursor);
    println!("BKLT un: {}", _unused);

    skip(cursor, 4);
    IdentBlock::BKLT("".to_string())
}

fn read_desc(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let channel_hash = read_u64_le(cursor);
    println!("DESC channel hash: {}", channel_hash);
    skip(cursor, 12);
    skip(cursor, 4);

    let num_points_set = read_u32_le(cursor);
    let num_points_scanned = read_u32_le(cursor);
    println!("DESC points set: {}", num_points_set);
    println!("DESC points got: {}", num_points_scanned);

    // SI32
    let matrix_type = read_matrix_string(cursor);
    println!("DESC matrix type: {}", matrix_type);

    let num_images = read_u32_le(cursor);
    println!("num images: {}", num_images);

    skip(cursor, 8);

    let num_points_set_alt = read_u32_le(cursor);
    println!("num points set alt: {}", num_points_set_alt);

    IdentBlock::DESC("".to_string())
}

fn read_data(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let len = read_u32_le(cursor);
    println!("DATA len: {}", len);

    let mut v = Vec::new();
    for _ in 0..len / 4 {
        v.push(read_u32_le(cursor));
    }
    println!("end of loop");
    println!("vec len: {}", &v[1]);
    // let a = read_matrix_type(cursor);
    // println!("a: {}", a);
    println!("end: {}", cursor.position());

    IdentBlock::DATA("".to_string())
}
