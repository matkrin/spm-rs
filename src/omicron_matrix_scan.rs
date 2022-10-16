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

#[derive(Debug)]
pub struct ScanData {
    pub datetime: DateTime<Utc>,
    pub desc: HashMap<String, u32>,
    pub img_data: Vec<u32>,
}


pub fn read_omicron_matrix_scanfile(filename: &str) -> ScanData {
    let bytes = read(filename).unwrap();
    let file_length = bytes.len();
    let mut cursor = Cursor::new(&bytes);

    let magic_header = read_magic_header(&mut cursor);
    assert_eq!(magic_header, "ONTMATRX0101");


    // println!("file length: {}", file_length);
    // let mut position = 0;
    // while position < file_length as u64 {
    //     let block = read_ident_block(&mut cursor);
    // }

    
    let scandata = ScanData {
        datetime: read_bklt(&mut cursor),
        desc: read_desc(&mut cursor),
        img_data:read_data(&mut cursor)
    };

    assert_eq!(cursor.position(), file_length as u64);
    scandata
}

// fn read_ident_block(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
//     let ident: String = read_matrix_type(cursor);
//     // println!("ident: {}", ident);
//
//     match ident.as_str() {
//         "BKLT" => read_bklt(cursor),
//         "DESC" => read_desc(cursor),
//         "DATA" => read_data(cursor),
//         _ => unimplemented!(),
//     }
// }

fn read_bklt(cursor: &mut Cursor<&Vec<u8>>) -> DateTime<Utc> {
    let ident: String = read_matrix_type(cursor);
    assert_eq!(ident, "BKLT");
    let _len = read_u32_le(cursor);
    // println!("BKLT len: {}", _len);

    // Time when image finished
    let time = read_u32_le(cursor);
    // println!("BKLT time: {}", time);
    let _unused = read_u32_le(cursor);
    // println!("BKLT un: {}", _unused);

    skip(cursor, 4);

    Utc.timestamp(time as i64, 0)
    // println!("Datetime: {}", t.with_timezone(&FixedOffset::east(1*3600)).to_string());
    // IdentBlock::BKLT(t)
}

fn read_desc(cursor: &mut Cursor<&Vec<u8>>) -> HashMap<String, u32> {
    let ident: String = read_matrix_type(cursor);
    assert_eq!(ident, "DESC");
    let _channel_hash = read_u64_le(cursor);
    skip(cursor, 16);

    let mut hm: HashMap<String, u32> = HashMap::new();
    hm.insert("num_points_set".to_string(), read_u32_le(cursor));
    hm.insert("num_points_scanned".to_string(), read_u32_le(cursor));

    // "SI32" don't know how this is useful
    let _matrix_type = read_matrix_string(cursor);

    // It seems also empty channels with no data listed here
    hm.insert("num_img_channels".to_string(), read_u32_le(cursor));

    skip(cursor, 8);

    hm.insert("num_points_set_alt".to_string(), read_u32_le(cursor));

    hm
    // println!("DESC hm: {:#?}", hm);
    // IdentBlock::DESC(hm)
}

// TODO: num images
fn read_data(cursor: &mut Cursor<&Vec<u8>>) -> Vec<u32> {

    let ident: String = read_matrix_type(cursor);
    assert_eq!(ident, "DATA");
    let len = read_u32_le(cursor);
    // println!("DATA len: {}", len);
    let img_data_len = len / size_of::<u32>() as u32;


    let mut img_data = Vec::with_capacity(img_data_len as usize);
    // TODO: this is the data for all channels
    // DESC shows 4 image channels but the data points here are 2 x 160_000 (2 400x400 pixel images)
    for _ in 0..img_data_len {
        img_data.push(read_u32_le(cursor));
    }

    img_data
    // return all data here, then with info from paramfile split it for use in seperate images
    // IdentBlock::DATA(img_data)
}
