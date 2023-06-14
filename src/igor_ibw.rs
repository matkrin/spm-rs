use anyhow::Result;
use std::{
    fs::read,
    io::{Cursor, Read},
};

use crate::utils::Bytereading;

#[derive(Debug)]
pub struct Ibw {
    // creation_date:
    // mod_date:
    pub npnts: i32,
    pub bname: String,
    pub n_dim: [i32; 4],
    pub x_step: [f64; 4],
    pub x_start: [f64; 4],
    pub data_units: String,
    pub data: NumericData,
    pub note: String,
    pub extended_data_units: Option<String>,
    pub dim_e_units: Option<Vec<String>>,
    pub dim_labels: Option<Vec<String>>,
}

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
    pub npnts: i32,      // Number of data points in wave.
    pub a_modified: i16, // Used in memory only. Write zero. Ignore on read.
    pub hs_a: f64,
    pub hs_b: f64,        // X value for point p = hsA*p + hsB
    pub w_modified: i16,  // Used in memory only. Write zero. Ignore on read.
    pub sw_modified: i16, // Used in memory only. Write zero. Ignore on read.
    pub fs_valid: i16,    // True if full scale values have meaning.
    pub top_full_scale: f64,
    pub bot_full_scale: f64, // The min full scale value for wave.
    pub use_bits: char,      // Used in memory only. Write zero. Ignore on read.
    pub kind_bits: char,     // Reserved. Write zero. Ignore on read.
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

// TODO use generics instead
#[derive(Debug)]
pub enum NumericData {
    Int8(Vec<i8>),
    Int16(Vec<i16>),
    Int32(Vec<i32>),

    Uint8(Vec<u8>),
    Uint16(Vec<u16>),
    Uint32(Vec<u32>),

    Float32(Vec<f32>),
    Float64(Vec<f64>),
}

pub fn read_ibw(filename: &str) -> Result<Ibw> {
    let bytes = read(filename)?;
    let file_len = bytes.len();
    let mut cursor = Cursor::new(bytes.as_slice());
    let version = cursor.read_i16_le();
    cursor.set_position(0);

    let bin_header = match version {
        2 => read_bin_header_2(&mut cursor),
        5 => read_bin_header_5(&mut cursor),
        _ => unreachable!("Not a version 2 or version 5 bin header"),
    };

    let wave_header = match version {
        2 => read_wave_header_2(&mut cursor),
        5 => read_wave_header_5(&mut cursor),
        _ => unreachable!("Not a version 2 or version 5 wave header"),
    };

    let npnts = match &wave_header {
        WaveHeader::V2(wh) => wh.npnts,
        WaveHeader::V5(wh) => wh.npnts,
    };

    let type_ = match &wave_header {
        WaveHeader::V2(wh) => wh.type_,
        WaveHeader::V5(wh) => wh.type_,
    };

    // TODO reshape data maybe
    let data = read_numeric_data(&mut cursor, type_, npnts);

    // version 1,2,3 have 16 bytes of padding after numeric wave data
    if version == 1 || version == 2 || version == 3 {
        let pos = cursor.position();
        cursor.set_position(pos + 16);
    }

    // Optional Data
    // v1: no optional data
    // v2: wave note data
    // v3: wave note data, wave dependency formula
    // v5: wave dependency formula, wave note data, extended data units data, extended dimension units data, dimension label data, String indices used for text waves only

    let note = read_note(&mut cursor, &bin_header);
    let extended_data_units = read_extended_data_units(&mut cursor, &bin_header);
    let dim_e_units = read_dim_e_units(&mut cursor, &bin_header);
    let dim_labels = read_dim_labels(&mut cursor, &bin_header);
    let bname = match &wave_header {
        WaveHeader::V2(wh) => wh.bname.trim_matches(char::from(0)).to_string(),
        WaveHeader::V5(wh) => wh.bname.trim_matches(char::from(0)).to_string(),
    };
    let n_dim = match &wave_header {
        WaveHeader::V2(wh) => [wh.npnts, 0, 0, 0],
        WaveHeader::V5(wh) => wh.n_dim,
    };
    let x_step = match &wave_header {
        WaveHeader::V2(wh) => [wh.hs_a, 0_f64, 0_f64, 0_f64],
        WaveHeader::V5(wh) => wh.sf_a,
    };
    let x_start = match &wave_header {
        WaveHeader::V2(wh) => [wh.hs_b, 0_f64, 0_f64, 0_f64],
        WaveHeader::V5(wh) => wh.sf_b,
    };
    let data_units = match &wave_header {
        WaveHeader::V2(wh) => wh.data_units.trim_matches(char::from(0)).to_string(),
        WaveHeader::V5(wh) => wh.data_units.trim_matches(char::from(0)).to_string(),
    };

    Ok(Ibw {
        npnts,
        bname,
        n_dim,
        x_step,
        x_start,
        data_units,
        data,
        note,
        extended_data_units,
        dim_e_units,
        dim_labels,
    })
}

fn read_note(cursor: &mut Cursor<&[u8]>, bin_header: &BinHeader) -> String {
    let note_size = match bin_header {
        BinHeader::V2(bh) => bh.note_size,
        BinHeader::V5(bh) => bh.note_size,
        _ => unreachable!("Only version 2 and version 5 bin headers implemented"),
    };

    if note_size != 0 {
        cursor.read_string(note_size as usize).replace("\r", "\n")
    } else {
        "".to_string()
    }
}

fn read_extended_data_units(cursor: &mut Cursor<&[u8]>, bin_header: &BinHeader) -> Option<String> {
    // extended data units
    match bin_header {
        BinHeader::V5(bh) => {
            if bh.data_e_units_size != 0 {
                Some(cursor.read_string(bh.data_e_units_size as usize))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn read_dim_e_units(cursor: &mut Cursor<&[u8]>, bin_header: &BinHeader) -> Option<Vec<String>> {
    match bin_header {
        BinHeader::V5(bh) => Some(
            bh.dim_e_units_size
                .iter()
                .map(|i| {
                    if *i != 0 {
                        cursor.read_string(*i as usize)
                    } else {
                        "".to_string()
                    }
                })
                .collect::<Vec<String>>(),
        ),
        _ => None,
    }
}

fn read_dim_labels(cursor: &mut Cursor<&[u8]>, bin_header: &BinHeader) -> Option<Vec<String>> {
    match bin_header {
        BinHeader::V5(bh) => Some(
            bh.dim_labels_size
                .iter()
                .map(|i| {
                    if *i != 0 {
                        cursor.read_string(*i as usize)
                    } else {
                        "".to_string()
                    }
                })
                .collect::<Vec<String>>(),
        ),
        _ => None,
    }
}

fn read_bin_header_2(cursor: &mut Cursor<&[u8]>) -> BinHeader {
    let version = cursor.read_i16_le();
    let wfm_size = cursor.read_i32_le();
    let note_size = cursor.read_i32_le();
    let pict_size = cursor.read_i32_le();
    let checksum = cursor.read_i16_le();

    BinHeader::V2(BinHeader2 {
        version,
        wfm_size,
        note_size,
        pict_size,
        checksum,
    })
}

fn read_wave_header_2(cursor: &mut Cursor<&[u8]>) -> WaveHeader {
    let type_ = cursor.read_i16_le();
    let next = cursor.read_u32_le();
    let bname = cursor.read_string(20);
    let wh_version = cursor.read_i16_le();
    let src_fldr = cursor.read_i16_le();
    let file_name = cursor.read_u32_le();
    let data_units = cursor.read_string(4);
    let x_units = cursor.read_string(4);
    let npnts = cursor.read_i32_le();
    let a_modified = cursor.read_i16_le();
    let hs_a = cursor.read_f64_le();
    let hs_b = cursor.read_f64_le();
    let w_modified = cursor.read_i16_le();
    let sw_modified = cursor.read_i16_le();
    let fs_valid = cursor.read_i16_le();
    let top_full_scale = cursor.read_f64_le();
    let bot_full_scale = cursor.read_f64_le();

    let mut use_bits = [0_u8; 1];
    cursor.read_exact(&mut use_bits).unwrap();

    let mut kind_bits = [0_u8; 1];
    cursor.read_exact(&mut kind_bits).unwrap();

    let formula = cursor.read_u32_le();
    let dep_id = cursor.read_i32_le();
    let creation_date = cursor.read_u32_le();
    let w_unused = cursor.read_string(2);
    let mod_date = cursor.read_u32_le();
    let wave_note_h = cursor.read_u32_le();

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
    let version = cursor.read_i16_le();
    let checksum = cursor.read_i16_le();
    let wfm_size = cursor.read_i32_le();
    let formula_size = cursor.read_i32_le();
    let note_size = cursor.read_i32_le();
    let data_e_units_size = cursor.read_i32_le();

    let mut dim_e_units_size = [0; 4];
    for i in dim_e_units_size.iter_mut() {
        *i = cursor.read_i32_le();
    }

    let mut dim_labels_size = [0; 4];
    for i in dim_labels_size.iter_mut() {
        *i = cursor.read_i32_le();
    }

    let s_indices_size = cursor.read_i32_le();
    let options_size_1 = cursor.read_i32_le();
    let options_size_2 = cursor.read_i32_le();

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
    let next = cursor.read_u32_le();
    let creation_date = cursor.read_u32_le();
    let mod_date = cursor.read_u32_le();
    let npnts = cursor.read_i32_le();
    let type_ = cursor.read_i16_le();
    let d_lock = cursor.read_i16_le();
    let whpad1 = cursor.read_string(6);
    let wh_version = cursor.read_i16_le();
    let bname = cursor.read_string(32);
    let whpad2 = cursor.read_i32_le();
    let data_folder = cursor.read_u32_le();

    let mut n_dim = [0; 4];
    for i in n_dim.iter_mut() {
        *i = cursor.read_i32_le();
    }

    let mut sf_a = [0_f64; 4];
    for i in sf_a.iter_mut() {
        *i = cursor.read_f64_le();
    }

    let mut sf_b = [0_f64; 4];
    for i in sf_b.iter_mut() {
        *i = cursor.read_f64_le();
    }

    let data_units = cursor.read_string(4);

    let mut dim_units = [[0_u8; 4]; 4];
    for i in dim_units.iter_mut() {
        cursor.read_exact(i).unwrap();
    }
    let fs_valid = cursor.read_i16_le();
    let whpad3 = cursor.read_i16_le();
    let top_full_scale = cursor.read_f64_le();
    let bot_full_scale = cursor.read_f64_le();
    let data_e_units = cursor.read_u32_le();

    let mut dim_e_units = [0_u32; 4];
    for i in dim_e_units.iter_mut() {
        *i = cursor.read_u32_le();
    }

    let mut dim_labels = [0_u32; 4];
    for i in dim_labels.iter_mut() {
        *i = cursor.read_u32_le();
    }

    let wave_note_h = cursor.read_u32_le();

    let mut wh_unused = [0_i32; 16];
    for i in wh_unused.iter_mut() {
        *i = cursor.read_i32_le();
    }
    let a_modified = cursor.read_i16_le();
    let w_modified = cursor.read_i16_le();
    let sw_modified = cursor.read_i16_le();

    let mut use_bits = [0_u8; 1];
    cursor.read_exact(&mut use_bits).unwrap();

    let mut kind_bits = [0_u8; 1];
    cursor.read_exact(&mut kind_bits).unwrap();
    let formula = cursor.read_u32_le();
    let dep_id = cursor.read_i32_le();
    let whpad4 = cursor.read_i16_le();
    let src_fldr = cursor.read_i16_le();
    let file_name = cursor.read_u32_le();
    let s_indeces = cursor.read_i32_le();

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

fn read_numeric_data(
    cursor: &mut Cursor<&[u8]>,
    data_type: i16,
    num_data_points: i32,
) -> NumericData {
    match data_type {
        0 => todo!("Text Waves"),
        1 => todo!("Complex"),
        2 => {
            let mut v = Vec::with_capacity((num_data_points / 4) as usize);
            for _ in 0..num_data_points {
                v.push(cursor.read_f32_le());
            }
            NumericData::Float32(v)
        }
        3 => todo!("Complex 64"),
        4 => {
            let mut v = Vec::with_capacity((num_data_points / 8) as usize);
            for _ in 0..num_data_points {
                v.push(cursor.read_f64_le());
            }
            NumericData::Float64(v)
        }
        5 => todo!("Complex 128"),
        8 => {
            let mut v = Vec::with_capacity((num_data_points) as usize);
            for _ in 0..num_data_points {
                v.push(cursor.read_i8_le());
            }
            NumericData::Int8(v)
        }
        9 => todo!("Complex Int8"),
        0x10 => {
            let mut v = Vec::with_capacity((num_data_points / 2) as usize);
            for _ in 0..num_data_points {
                v.push(cursor.read_i16_le());
            }
            NumericData::Int16(v)
        }
        0x11 => todo!("Complex Int16"),

        0x20 => {
            let mut v = Vec::with_capacity((num_data_points / 4) as usize);
            for _ in 0..num_data_points {
                v.push(cursor.read_i32_le());
            }
            NumericData::Int32(v)
        }
        0x21 => todo!("Complex Int32"),

        0x48 => {
            let mut v = Vec::with_capacity((num_data_points) as usize);
            for _ in 0..num_data_points {
                v.push(cursor.read_u8_le());
            }
            NumericData::Uint8(v)
        }
        0x49 => todo!("Complex UInt8"),

        0x50 => {
            let mut v = Vec::with_capacity((num_data_points / 2) as usize);
            for _ in 0..num_data_points {
                v.push(cursor.read_u16_le());
            }
            NumericData::Uint16(v)
        }
        0x51 => todo!("Complex UInt16 Data"),

        0x60 => {
            let mut v = Vec::with_capacity((num_data_points / 4) as usize);
            for _ in 0..num_data_points {
                v.push(cursor.read_u32_le());
            }
            NumericData::Uint32(v)
        }
        0x61 => todo!("Complex UInt32 Data"),
        _ => unreachable!(),
    }
}
