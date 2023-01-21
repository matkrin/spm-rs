use anyhow::Result;
use std::{
    fs::read,
    io::{Cursor, Read},
};

use crate::utils::{read_f32_le, read_f64_le, read_i16_le, read_i32_le, read_string, read_u32_le};

pub struct BinHeader1 {
    pub version: i16,  // Version number for backwards compatibility.
    pub wfm_size: i32, // The size of the WaveHeader2 data structure plus the wave data plus 16 bytes of padding.
    pub checksum: i16, // Checksum over this header and the wave header.
}

struct BinHeader2 {
    pub version: i16,   // Version number for backwards compatibility.
    pub wfm_size: i32, // The size of the WaveHeader2 data structure plus the wave data plus 16 bytes of padding.
    pub note_size: i32, // The size of the note text.
    pub pict_size: i32, // Reserved. Write zero. Ignore on read.
    pub checksum: i16, // Checksum over this header and the wave header.
}

struct BinHeader3 {
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

struct WaveHeader2 {
    type_: i16, // See types (e.g. NT_FP64) above. Zero for text waves.
    // struct WaveHeader2 **next,			// Used in memory only. Write zero. Ignore on read.

    // char bname[MAX_WAVE_NAME2+2],		// Name of wave plus trailing null. --- 31 + 2
    bname: String,
    wh_version: i16, // Write 0. Ignore on read.
    src_fldr: i16,   // Used in memory only. Write zero. Ignore on read.
    // Handle fileName,					// Used in memory only. Write zero. Ignore on read.

    // char dataUnits[MAX_UNIT_CHARS+1],	// Natural data units go here - null if none.
    data_units: String,
    // char xUnits[MAX_UNIT_CHARS+1],		// Natural x-axis units go here - null if none.
    x_units: String,

    npnts: i32, // Number of data points in wave.

    a_modified: i16, // Used in memory only. Write zero. Ignore on read.
    hs_a: f64,
    hs_b: f64, // X value for point p = hsA*p + hsB

    w_modified: i16,  // Used in memory only. Write zero. Ignore on read.
    sw_modified: i16, // Used in memory only. Write zero. Ignore on read.
    fs_valid: i16,    // True if full scale values have meaning.
    top_full_scale: f64,
    bot_full_scale: f64, // The min full scale value for wave.

    use_bits: char,  // Used in memory only. Write zero. Ignore on read.
    kind_bits: char, // Reserved. Write zero. Ignore on read.
    // void **formula,						// Used in memory only. Write zero. Ignore on read.
    dep_id: i32,        // Used in memory only. Write zero. Ignore on read.
    creation_date: u32, // DateTime of creation. Not used in version 1 files.
    // char wUnused[2],					// Reserved. Write zero. Ignore on read.
    w_unused: String,

    mod_date: u32, // DateTime of last modification.
    // Handle wave_note_h,					// Used in memory only. Write zero. Ignore on read.
    w_data: [f32; 4], // The start of the array of waveform data.
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

pub fn read_ibw(filename: &str) -> Result<()> {
    let bytes = read(filename)?;
    let file_len = bytes.len();
    println!("file len: {}", file_len);
    let mut cursor = Cursor::new(bytes.as_slice());

    let bin_header_5 = read_bin_header(&mut cursor);
    println!("bin_header_5: {:#?}", bin_header_5);
    println!("cursor position: {}", cursor.position());

    let wave_header_5 = read_wave_header_5(&mut cursor);
    println!("wave_header_5: {:#?}", wave_header_5);
    // let num_points_to_read = npnts * 4;
    // for i in 0..num_points_to_read / 4 {
    //
    // }
    for i in 0..wave_header_5.npnts {
        let n = read_f32_le(&mut cursor);
        println!("data point {}: {}", i, n);
    }

    // float wData[1],						// The start of the array of data. Must be 64 bit aligned.
    // w_data: f32,
    // let w_data = read_f32_le(&mut cursor);
    // println!("w_data: {}", w_data);
    //
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    //
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    //
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    //
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    //
    // println!("pos: {}", cursor.position());
    // let n = read_string(&mut cursor, 15);
    // println!("next data: {}", n);
    // println!("pos: {}", cursor.position());
    //
    //
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    //
    // println!("pos: {}", cursor.position());
    //
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    // let n = read_f32_le(&mut cursor);
    // println!("next data: {}", n);
    //
    // println!("pos: {}", cursor.position());

    Ok(())
}

fn read_bin_header(cursor: &mut Cursor<&[u8]>) -> BinHeader5 {
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

    BinHeader5 {
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
    }
}

fn read_wave_header_5(cursor: &mut Cursor<&[u8]>) -> WaveHeader5 {
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
    //
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

    WaveHeader5 {
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
    }
}
