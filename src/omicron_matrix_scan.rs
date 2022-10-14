use std::collections::HashMap;
use std::fs::read;
use std::io::Cursor;
use std::mem::size_of;
use std::str;

use chrono::prelude::*;
use chrono::Utc;

use crate::utils::{
    read_magic_header, read_matrix_string, read_matrix_type, read_u32_le, read_u64_le, skip,
};

pub fn read_omicron_matrix_scanfile(filename: &str) {
    let bytes = read(filename).unwrap();
    let mut cursor = Cursor::new(&bytes);
    let magic_header = read_magic_header(&mut cursor);
    assert_eq!(magic_header, "ONTMATRX0101");

    let file_length = bytes.len();
    // println!("file length: {}", file_length);
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
    BKLT(DateTime<Utc>),
    DESC(HashMap<String, u32>),
    DATA(Vec<u32>),
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
    // println!("BKLT len: {}", _len);

    // Time when image finished
    let time = read_u32_le(cursor);
    println!("BKLT time: {}", time);
    let _unused = read_u32_le(cursor);
    // println!("BKLT un: {}", _unused);

    skip(cursor, 4);

    let t = Utc.timestamp(time as i64, 0);
    // println!("Datetime: {}", t.with_timezone(&FixedOffset::east(1*3600)).to_string());
    IdentBlock::BKLT(t)
}

fn read_desc(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let channel_hash = read_u64_le(cursor);
    println!("DESC channel hash: {}", channel_hash);
    skip(cursor, 16);

    let mut hm: HashMap<String, u32> = HashMap::new();
    hm.insert("num_points_set".to_string(), read_u32_le(cursor));
    hm.insert("num_points_scanned".to_string(), read_u32_le(cursor));

    // "SI32" don't know how this is useful
    let _matrix_type = read_matrix_string(cursor);
    println!("DESC matrix type: {}", _matrix_type);

    // It seems also empty channels with no data listed here
    hm.insert("num_img_channels".to_string(), read_u32_le(cursor));

    skip(cursor, 8);

    hm.insert("num_points_set_alt".to_string(), read_u32_le(cursor));

    // println!("DESC hm: {:#?}", hm);
    IdentBlock::DESC(hm)
}

// TODO: num images
fn read_data(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let len = read_u32_le(cursor);
    // println!("DATA len: {}", len);
    let vec_len = len / size_of::<u32>() as u32;

    let mut img_data = Vec::with_capacity(vec_len as usize);
    // TODO: this is the data for all channels
    // DESC shows 4 image channels but the data points here are 2 x 160_000 (2 400x400 pixel images)
    for _ in 0..vec_len {
        img_data.push(read_u32_le(cursor));
    }

    // return all data here, then with info from paramfile split it for use in seperate images
    IdentBlock::DATA(img_data)
}
