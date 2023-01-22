use anyhow::Result;
use std::{
    fs::read,
    io::{Cursor, Read},
};

use crate::utils::{read_f32_le, read_f64_le, read_i16_le, read_i32_le, read_string, read_u32_le};

#[derive(Debug)]
pub enum BinHeader {
    V1(BinHeader1),
    V2(BinHeader2),
    V4(BinHeader3),
    V5(BinHeader5),
}

#[derive(Debug)]
pub struct BinHeader1 {
    pub version: i16,  // Version number for backwards compatibility.
    pub wfm_size: i32, // The size of the WaveHeader2 data structure plus the wave data plus 16 bytes of padding.
    pub checksum: i16, // Checksum over this header and the wave header.
}

#[derive(Debug)]
pub struct BinHeader2 {
    pub version: i16,   // Version number for backwards compatibility.
    pub wfm_size: i32, // The size of the WaveHeader2 data structure plus the wave data plus 16 bytes of padding.
    pub note_size: i32, // The size of the note text.
    pub pict_size: i32, // Reserved. Write zero. Ignore on read.
    pub checksum: i16, // Checksum over this header and the wave header.
}

#[derive(Debug)]
pub struct BinHeader3 {
    pub version: i16,      // Version number for backwards compatibility.
    pub wfm_size: i32, // The size of the WaveHeader2 data structure plus the wave data plus 16 bytes of padding.
    pub note_size: i32, // The size of the note text.
    pub formula_size: i32, // The size of the dependency formula, if any.
    pub pict_size: i32, // Reserved. Write zero. Ignore on read.
    pub checksum: i32, // Checksum over this header and the wave header.
}

#[derive(Debug)]
pub struct BinHeader5 {
    pub version: i16,               // Version number for backwards compatibility.
    pub checksum: i16,              // Checksum over this header and the wave header.
    pub wfm_size: i32, // The size of the WaveHeader5 data structure plus the wave data.
    pub formula_size: i32, // The size of the dependency formula, if any.
    pub note_size: i32, // The size of the note text.
    pub data_e_units_size: i32, // The size of optional extended data units.
    pub dim_e_units_size: [i32; 4], // The size of optional extended dimension units.
    pub dim_labels_size: [i32; 4], // The size of optional dimension labels.
    pub s_indices_size: i32, // The size of string indicies if this is a text wave.
    pub options_size_1: i32, // Reserved. Write zero. Ignore on read.
    pub options_size_2: i32, // Reserved. Write zero. Ignore on read.
}

// #define MAX_WAVE_NAME2 18	// Maximum length of wave name in version 1 and 2 files. Does not include the trailing null.
// #define MAX_WAVE_NAME5 31	// Maximum length of wave name in version 5 files. Does not include the trailing null.
// #define MAX_UNIT_CHARS 3
//
//
// #define NT_CMPLX 1			// Complex numbers.
// #define NT_FP32 2			// 32 bit fp numbers.
// #define NT_FP64 4			// 64 bit fp numbers.
// #define NT_I8 8				// 8 bit signed integer. Requires Igor Pro 2.0 or later.
// #define NT_I16 	0x10		// 16 bit integer numbers. Requires Igor Pro 2.0 or later.
// #define NT_I32 	0x20		// 32 bit integer numbers. Requires Igor Pro 2.0 or later.
// #define NT_UNSIGNED 0x40	// Makes above signed integers unsigned. Requires Igor Pro 3.0 or later.
//

#[derive(Debug)]
pub enum WaveHeader {
    V2(WaveHeader2),
    V5(WaveHeader5),
}

#[derive(Debug)]
pub struct WaveHeader2 {
    pub type_: i16, // See types (e.g. NT_FP64) above. Zero for text waves.
    pub next: u32,
    pub bname: String,
    pub wh_version: i16, // Write 0. Ignore on read.
    pub src_fldr: i16,   // Used in memory only. Write zero. Ignore on read.
    pub file_name: u32,
    pub data_units: String,
    pub x_units: String,
    pub npnts: i32, // Number of data points in wave.
    pub a_modified: i16, // Used in memory only. Write zero. Ignore on read.
    pub hs_a: f64,
    pub hs_b: f64, // X value for point p = hsA*p + hsB
    pub w_modified: i16,  // Used in memory only. Write zero. Ignore on read.
    pub sw_modified: i16, // Used in memory only. Write zero. Ignore on read.
    pub fs_valid: i16,    // True if full scale values have meaning.
    pub top_full_scale: f64,
    pub bot_full_scale: f64, // The min full scale value for wave.
    pub use_bits: char,  // Used in memory only. Write zero. Ignore on read.
    pub kind_bits: char, // Reserved. Write zero. Ignore on read.
    pub formula: u32,
    pub dep_id: i32,        // Used in memory only. Write zero. Ignore on read.
    pub creation_date: u32, // DateTime of creation. Not used in version 1 files.
    pub w_unused: String,
    pub mod_date: u32, // DateTime of last modification.
    pub wave_note_h: u32,
}

#[derive(Debug)]
pub struct WaveHeader5 {
    pub next: u32,
    pub creation_date: u32,
    pub mod_date: u32,
    pub npnts: i32,
    pub type_: i16,
    pub d_lock: i16,
    pub whpad1: String,
    pub wh_version: i16,
    pub bname: String,
    pub whpad2: i32,
    pub data_folder: u32,
    pub n_dim: [i32; 4],
    pub sf_a: [f64; 4],
    pub sf_b: [f64; 4],
    pub data_units: String,
    pub dim_units: [[u8; 4]; 4],
    pub fs_valid: i16,
    pub whpad3: i16,
    pub top_full_scale: f64,
    pub bot_full_scale: f64,
    pub data_e_units: u32,
    pub dim_e_units: [u32; 4],
    pub dim_labels: [u32; 4],
    pub wave_note_h: u32,
    pub wh_unused: [i32; 16],
    pub a_modified: i16,
    pub w_modified: i16,
    pub sw_modified: i16,
    pub use_bits: char,
    pub kind_bits: char,
    pub formula: u32,
    pub dep_id: i32,
    pub whpad4: i16,
    pub src_fldr: i16,
    pub file_name: u32,
    pub s_indeces: i32, 
}

#[derive(Debug)]
pub enum NumericData {
    Int8(Vec<i8>),
    Int16(Vec<i16>),
    Int32(Vec<i32>),
    Float32(Vec<f32>),
    Float64(Vec<f64>),
}

pub fn read_ibw(filename: &str) -> Result<()> {
    let bytes = read(filename)?;
    let file_len = bytes.len();
    println!("file len: {}", file_len);
    let mut cursor = Cursor::new(bytes.as_slice());
    let version = read_i16_le(&mut cursor);
    cursor.set_position(0);


    let bin_header = match version {
        2 => read_bin_header_2(&mut cursor),
        5 => read_bin_header_5(&mut cursor),
        _ => unreachable!("Not a version 2 or version 5 bin header")
    };

    let wave_header = match version {
        2 => read_wave_header_2(&mut cursor),
        5 => read_wave_header_5(&mut cursor),
        _ => unreachable!("Not a version 2 or version 5 wave header"),
    };

    println!("bin_header: {:#?}", bin_header);
    println!("wave_header: {:#?}", wave_header);
    println!("cursor position: {}", cursor.position());

    let npnts = match &wave_header {
        WaveHeader::V2(wh) => wh.npnts,
        WaveHeader::V5(wh) => wh.npnts,
    };

    let type_ = match &wave_header {
        WaveHeader::V2(wh) => wh.type_,
        WaveHeader::V5(wh) => wh.type_,
    };


    let data = read_numeric_data(&mut cursor, type_, npnts);

    // println!("data: {:?}", data);
    match data {
        NumericData::Float64(v) => println!("{}", v.len()),
        _ => unreachable!(),
    }

    // version 2 has 16 bytes of padding
    match version {
        2 => {
            let pos = cursor.position();
            cursor.set_position(pos + 16);
        },
        5 => {},
        _ => unreachable!(),
    }

    let note_size = match bin_header {
        BinHeader::V2(bh) => bh.note_size,
        BinHeader::V5(bh) => bh.note_size,
        _ => unreachable!(),
    };

    let note = read_string(&mut cursor, note_size as usize);
    println!("note: {}", note.replace("\r", "\n"));
    println!("note len: {}", note.len());
    println!("pos: {}", cursor.position());


    Ok(())
}

fn read_bin_header_2(cursor: &mut Cursor<&[u8]>) -> BinHeader {
    let version = read_i16_le(cursor);
    let wfm_size = read_i32_le(cursor);
    let note_size = read_i32_le(cursor);
    let pict_size = read_i32_le(cursor);
    let checksum = read_i16_le(cursor);

    BinHeader::V2(BinHeader2 {
        version,
        wfm_size,
        note_size,
        pict_size,
        checksum,
    })
}

fn read_wave_header_2(cursor: &mut Cursor<&[u8]>) -> WaveHeader {
    let type_ = read_i16_le(cursor);
    let next = read_u32_le(cursor);
    let bname = read_string(cursor, 20);
    let wh_version = read_i16_le(cursor);
    let src_fldr = read_i16_le(cursor);
    let file_name = read_u32_le(cursor);
    let data_units = read_string(cursor, 4);
    let x_units = read_string(cursor, 4);
    let npnts = read_i32_le(cursor);
    let a_modified = read_i16_le(cursor);
    let hs_a = read_f64_le(cursor);
    let hs_b = read_f64_le(cursor);
    let w_modified = read_i16_le(cursor);
    let sw_modified = read_i16_le(cursor);
    let fs_valid = read_i16_le(cursor);
    let top_full_scale = read_f64_le(cursor);
    let bot_full_scale = read_f64_le(cursor);

    let mut use_bits = [0_u8; 1];
    cursor.read_exact(&mut use_bits).unwrap();

    let mut kind_bits = [0_u8; 1];
    cursor.read_exact(&mut kind_bits).unwrap();

    let formula = read_u32_le(cursor);
    let dep_id = read_i32_le(cursor);
    let creation_date = read_u32_le(cursor);
    let w_unused = read_string(cursor, 2);
    let mod_date = read_u32_le(cursor);
    let wave_note_h = read_u32_le(cursor);

    WaveHeader::V2(WaveHeader2 {
        type_,
        next,
        bname,
        wh_version,
        src_fldr,
        file_name,
        data_units,
        x_units,
        npnts,
        a_modified,
        hs_a,
        hs_b,
        w_modified,
        sw_modified,
        fs_valid,
        top_full_scale,
        bot_full_scale,
        use_bits: use_bits[0] as char,
        kind_bits: kind_bits[0] as char,
        formula,
        dep_id,
        creation_date,
        w_unused,
        mod_date,
        wave_note_h,
    })
}


fn read_bin_header_5(cursor: &mut Cursor<&[u8]>) -> BinHeader {
    let version = read_i16_le(cursor);
    let checksum = read_i16_le(cursor);
    let wfm_size = read_i32_le(cursor);
    let formula_size = read_i32_le(cursor);
    let note_size = read_i32_le(cursor);
    let data_e_units_size = read_i32_le(cursor);

    let mut dim_e_units_size = [0; 4];
    for i in dim_e_units_size.iter_mut() {
        *i = read_i32_le(cursor);
    }

    let mut dim_labels_size = [0; 4];
    for i in dim_labels_size.iter_mut() {
        *i = read_i32_le(cursor);
    }

    let s_indices_size = read_i32_le(cursor);
    let options_size_1 = read_i32_le(cursor);
    let options_size_2 = read_i32_le(cursor);

    BinHeader::V5(BinHeader5 {
        version,
        checksum,
        wfm_size,
        formula_size,
        note_size,
        data_e_units_size,
        dim_e_units_size,
        dim_labels_size,
        s_indices_size,
        options_size_1,
        options_size_2,
    })
}

fn read_wave_header_5(cursor: &mut Cursor<&[u8]>) -> WaveHeader {
    let next = read_u32_le(cursor);
    let creation_date = read_u32_le(cursor);
    let mod_date = read_u32_le(cursor);
    let npnts = read_i32_le(cursor);
    let type_ = read_i16_le(cursor);
    let d_lock = read_i16_le(cursor);
    let whpad1 = read_string(cursor, 6);
    let wh_version = read_i16_le(cursor);
    let bname = read_string(cursor, 32);
    let whpad2 = read_i32_le(cursor);
    let data_folder = read_u32_le(cursor);

    let mut n_dim = [0; 4];
    for i in n_dim.iter_mut() {
        *i = read_i32_le(cursor);
    }

    let mut sf_a = [0_f64; 4];
    for i in sf_a.iter_mut() {
        *i = read_f64_le(cursor);
    }

    let mut sf_b = [0_f64; 4];
    for i in sf_b.iter_mut() {
        *i = read_f64_le(cursor);
    }

    let data_units = read_string(cursor, 4);

    let mut dim_units = [[0_u8; 4]; 4];
    for i in dim_units.iter_mut() {
        cursor.read_exact(i).unwrap();
    }
    let fs_valid = read_i16_le(cursor);
    let whpad3 = read_i16_le(cursor);
    let top_full_scale = read_f64_le(cursor);
    let bot_full_scale = read_f64_le(cursor);
    let data_e_units = read_u32_le(cursor);

    let mut dim_e_units = [0_u32; 4];
    for i in dim_e_units.iter_mut() {
        *i = read_u32_le(cursor);
    }

    let mut dim_labels = [0_u32; 4];
    for i in dim_labels.iter_mut() {
        *i = read_u32_le(cursor);
    }

    let wave_note_h = read_u32_le(cursor);
    
    let mut wh_unused = [0_i32; 16];
    for i in wh_unused.iter_mut() {
        *i = read_i32_le(cursor);
    }
    let a_modified = read_i16_le(cursor);
    let w_modified = read_i16_le(cursor);
    let sw_modified = read_i16_le(cursor);
    
    let mut use_bits = [0_u8; 1];
    cursor.read_exact(&mut use_bits).unwrap();

    let mut kind_bits = [0_u8; 1];
    cursor.read_exact(&mut kind_bits).unwrap();
    let formula = read_u32_le(cursor);
    let dep_id = read_i32_le(cursor);
    let whpad4 = read_i16_le(cursor);
    let src_fldr = read_i16_le(cursor);
    let file_name = read_u32_le(cursor);
    let s_indeces = read_i32_le(cursor);

    WaveHeader::V5(WaveHeader5 {
        next,
        creation_date,
        mod_date,
        npnts,
        type_,
        d_lock,
        whpad1,
        wh_version,
        bname,
        whpad2,
        data_folder,
        n_dim,
        sf_a,
        sf_b,
        data_units,
        dim_units,
        fs_valid,
        whpad3,
        top_full_scale,
        bot_full_scale,
        data_e_units,
        dim_e_units,
        dim_labels,
        wave_note_h,
        wh_unused,
        a_modified,
        w_modified,
        sw_modified,
        use_bits: use_bits[0] as char,
        kind_bits: kind_bits[0] as char,
        formula,
        dep_id,
        whpad4,
        src_fldr,
        file_name,
        s_indeces,
    })
}

fn read_numeric_data(cursor: &mut Cursor<&[u8]>, data_type: i16, num_data_points: i32) -> NumericData {
    match data_type {
        0 => todo!("Text Waves"),
        1 => todo!("Complex"),
        2 => {
            let mut v = Vec::with_capacity((num_data_points / 4) as usize);
            for _ in 0..num_data_points {
                v.push(read_f32_le(cursor));
            }
            NumericData::Float32(v)
        },
        3 => todo!("Complex 64"),
        4 => {
            let mut v = Vec::with_capacity((num_data_points / 8) as usize);
            for _ in 0..num_data_points {
                v.push(read_f64_le(cursor));
            }
            NumericData::Float64(v)
        },
        5 => todo!("Complex 128"),
        8 => todo!("Int8 Data"),
        9 => todo!("Complex Int8"),
        0x10 => {
            let mut v = Vec::with_capacity((num_data_points / 2) as usize);
            for _ in 0..num_data_points {
                v.push(read_i16_le(cursor));
            }
            NumericData::Int16(v)
        },
        0x11 => todo!("Complex Int16"),

        0x20 => {
            let mut v = Vec::with_capacity((num_data_points / 4) as usize);
            for _ in 0..num_data_points {
                v.push(read_i32_le(cursor));
            }
            NumericData::Int32(v)
        },
        0x21 => todo!("Complex Int32"),

        0x48 => todo!("UInt8 Data"),
        0x49 => todo!("Complex UInt8"),

        0x50 => todo!("UInt16 Data"),
        0x51 => todo!("Complex UInt16 Data"),

        0x60 => todo!("UInt32 Data"),
        0x61 => todo!("Complex UInt32 Data"),
        _ => unreachable!(),
    }
}
