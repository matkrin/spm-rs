use std::collections::HashMap;
use std::fs::read;
use std::io::Cursor;
use std::str;

use crate::utils::{
    read_f64_le, read_magic_header, read_matrix_string, read_matrix_type, read_string, read_u32_le,
    skip,
};

#[derive(Debug)]
pub enum IdentBlock {
    META(HashMap<String, String>),
    EXPD(String),
    FSEQ(String),
    EXPS(String),
    GENL(String),
    EEPA(HashMap<String, MatrixType>),
    INCI(String),
    MARK(String),
    VIEW(String),
    PROC(String),
    PMOD(HashMap<String, MatrixType>),
    CCSY(String),
    BREF(String),
    EOED(bool),
    INST(HashMap<String, String>),
    CNXS(HashMap<String, String>),
    DICT(HashMap<String, u32>),
    CHCS(String),
    XFER(HashMap<String, MatrixType>),
    SCAN(String),
}

#[derive(Debug)]
pub enum MatrixType {
    BOOL(u32),
    LONG(u32),
    STRG(String),
    DOUB(f64),
}

#[derive(Debug)]
pub struct ParamInfo {
    current: f64,
    bias: f64,
    xsize: f64,
    ysize: f64,
    xres: u32,
    yres: u32,
    rotation: u32,
    raster_time: f64,
    xoffset: f64,
    yoffset: f64,
    xretrace: bool,
    yretrace: bool,
}

static CURRENT: &'static str = "Regulator.Setpoint_1 [Ampere]";
static _CURRENT_ALT: &'static str = "Regulator.Alternate_Setpoint_1 [Ampere]";
static BIAS: &'static str = "GapVoltageControl.Voltage [Volt]";
static XSIZE: &'static str = "XYScanner.Width [Meter]";
static YSIZE: &'static str = "XYScanner.Height [Meter]";
static XRES: &'static str = "XYScanner.Points [Count]";
static YRES: &'static str = "XYScanner.Lines [Count]";
static ROTATION: &'static str = "XYScanner.Angle [Degree]";
static RASTER_TIME: &'static str = "XYScanner.Raster_Time [Second]";
static XOFFSET: &'static str = "XYScanner.X_Offset [Meter]";
static YOFFSET: &'static str = "XYScanner.Y_Offset [Meter]";
static XRETRACE: &'static str = "XYScanner.X_Retrace [--]";
static YRETRACE: &'static str = "XYScanner.Y_Retrace [--]";

pub fn read_omicron_matrix_paramfile(filename: &str) -> ParamInfo {
    let bytes = read(filename).unwrap();
    let mut cursor = Cursor::new(&bytes);
    let magic_header = read_magic_header(&mut cursor);
    assert_eq!(magic_header, "ONTMATRX0101");

    let file_length = bytes.len();

    let mut current = 0.0;
    let mut bias = 0.0;
    let mut xsize = 0.0;
    let mut ysize = 0.0;
    let mut xres: u32 = 0;
    let mut yres: u32 = 0;
    let mut rotation: u32 = 0;
    let mut raster_time = 0.0;
    let mut xoffset = 0.0;
    let mut yoffset = 0.0;
    let mut xretrace = false;
    let mut yretrace = false;

    let mut position = 0;
    while position < file_length as u64 {

        // 1. read EEPA which gives initial values, in EEPA all keys should be in one Hashmap
        // 2. change initial values if the PMOD with the key for this value appears
        // 3. break if BREF with filename to look appears
        // => therefore values are always the ones from last PMOD, which should be right

        let block = read_ident_block(&mut cursor);
        match block {
            IdentBlock::EEPA(hm) => {
                if let MatrixType::DOUB(x) = hm[CURRENT] {
                    current = x;
                }
                if let MatrixType::DOUB(x) = hm[BIAS] {
                    bias = x;
                }
                if let MatrixType::DOUB(x) = hm[XSIZE] {
                    xsize = x;
                }
                if let MatrixType::DOUB(x) = hm[YSIZE] {
                    ysize = x;
                }
                if let MatrixType::LONG(x) = hm[XRES] {
                    xres = x;
                }
                if let MatrixType::LONG(x) = hm[YRES] {
                    yres = x;
                }
                if let MatrixType::LONG(x) = hm[ROTATION] {
                    rotation = x;
                }
                if let MatrixType::DOUB(x) = hm[RASTER_TIME] {
                    raster_time = x;
                }
                if let MatrixType::DOUB(x) = hm[XOFFSET] {
                    xoffset = x;
                }
                if let MatrixType::DOUB(x) = hm[YOFFSET] {
                    yoffset = x;
                }
                if let MatrixType::BOOL(x) = hm[XRETRACE] {
                    xretrace = if x == 0 { false } else { true };
                }
                if let MatrixType::BOOL(x) = hm[YRETRACE] {
                    yretrace = if x == 0 { false } else { true };
                }
            }
            IdentBlock::PMOD(hm) => {
                if hm.contains_key(CURRENT) {
                    if let MatrixType::DOUB(x) = hm[CURRENT] {
                        current = x;
                    }
                } else if hm.contains_key(BIAS) {
                    if let MatrixType::DOUB(x) = hm[BIAS] {
                        bias = x;
                    }
                } else if hm.contains_key(XSIZE) {
                    if let MatrixType::DOUB(x) = hm[XSIZE] {
                        xsize = x;
                    }
                } else if hm.contains_key(YSIZE) {
                    if let MatrixType::DOUB(x) = hm[YSIZE] {
                        ysize = x;
                    }
                } else if hm.contains_key(ROTATION) {
                    if let MatrixType::LONG(x) = hm[ROTATION] {
                        rotation = x;
                    }
                } else if hm.contains_key(RASTER_TIME) {
                    if let MatrixType::DOUB(x) = hm[RASTER_TIME] {
                        raster_time = x;
                    }
                } else if hm.contains_key(XOFFSET) {
                    if let MatrixType::DOUB(x) = hm[XOFFSET] {
                        xoffset = x;
                    }
                } else if hm.contains_key(YOFFSET) {
                    if let MatrixType::DOUB(x) = hm[YOFFSET] {
                        yoffset = x;
                    }
                } else if hm.contains_key(XRETRACE) {
                    if let MatrixType::BOOL(x) = hm[XRETRACE] {
                        xretrace = if x == 0 { false } else { true };
                    }
                } else if hm.contains_key(YRETRACE) {
                    if let MatrixType::BOOL(x) = hm[YRETRACE] {
                        yretrace = if x == 0 { false } else { true };
                    }
                }
            }
            IdentBlock::BREF(x) => {
                println!("f: {}", x);
                
                if x ==  "20201111--4_1.Z_mtrx" {
                    break;
                }
            }
            _ => continue,
        };
        position = cursor.position();
    }

    ParamInfo {
        current,
        bias,
        xsize,
        ysize,
        xres,
        yres,
        rotation,
        raster_time,
        xoffset,
        yoffset,
        xretrace,
        yretrace,
    }
}

// returns a Vec of all IdentBlock in the paramfile
pub fn read_omicron_matrix_paramfile_full(filename: &str) -> Vec<IdentBlock> {
    let bytes = read(filename).unwrap();
    let mut cursor = Cursor::new(&bytes);
    let magic_header = read_magic_header(&mut cursor);
    assert_eq!(magic_header, "ONTMATRX0101");

    let file_length = bytes.len();
    let mut position = 0;
    let mut v = Vec::new();
    while position < file_length as u64 {
        v.push(read_ident_block(&mut cursor));
        position = cursor.position();
    }
    // println!("v: {:#?}", v);
    assert_eq!(cursor.position(), file_length as u64);
    v
}

fn read_ident_block(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let ident: String = read_matrix_type(cursor);
    // println!("ident: {}", ident);

    match ident.as_str() {
        "META" => read_meta(cursor),
        "EXPD" => read_expd(cursor),
        "FSEQ" => read_fseq(cursor),
        "EXPS" => read_exps(cursor),
        "GENL" => read_genl(cursor),
        "EEPA" => read_eepa(cursor),
        "INCI" => read_inci(cursor),
        "MARK" => read_mark(cursor),
        "VIEW" => read_view(cursor),
        "PROC" => read_proc(cursor),
        "PMOD" => read_pmod(cursor),
        "CCSY" => read_ccsy(cursor),
        "BREF" => read_bref(cursor),
        "EOED" => read_eoed(cursor),
        "INST" => read_inst(cursor),
        "CNXS" => read_cnxs(cursor),
        "DICT" => read_dict(cursor),
        "CHCS" => read_chcs(cursor),
        "SCAN" => read_scan(cursor),
        "XFER" => read_xfer(cursor),
        _ => unimplemented!(),
    }
}

// META
fn read_meta(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unused = read_u32_le(cursor);

    let mut hm: HashMap<String, String> = HashMap::new();
    hm.insert("program_name".to_string(), read_matrix_string(cursor));
    hm.insert("version".to_string(), read_matrix_string(cursor));

    skip(cursor, 4);

    hm.insert("profile".to_string(), read_matrix_string(cursor));
    hm.insert("user".to_string(), read_matrix_string(cursor));

    skip(cursor, 4);

    IdentBlock::META(hm)
}

//EXPD
fn read_expd(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unused = read_u32_le(cursor);

    skip(cursor, 4);

    let mut content = String::new();
    for _ in 0..7 {
        let s = read_matrix_string(cursor);
        content += &format!("\n{}", s);
    }
    // println!("EXPD: {}", content.trim());
    IdentBlock::EXPD(content.trim().to_owned())
}

// FSEQ
fn read_fseq(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    // let len = read_u32_le(cursor);
    // let time = read_u32_le(cursor);
    skip(cursor, 20);
    IdentBlock::FSEQ("".to_string())
}

// EXPS contains INST and CNXS
// therefore len of exps contains len of inst and CNXS
// reading of those blocks is also handled by read_ident_block
fn read_exps(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unused = read_u32_le(cursor);
    skip(cursor, 4);

    IdentBlock::EXPS("".to_string())
}

// GENL
// this block is nested first in EXPS
fn read_genl(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);

    let a = read_matrix_string(cursor);
    let b = read_matrix_string(cursor);
    let c = read_matrix_string(cursor);
    IdentBlock::GENL(format!("{}; {}; {}", a, b, c))
}

// INST
// this block is nested second in EXPS
fn read_inst(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let len = read_u32_le(cursor);

    let mut position = cursor.position();
    let end = position + len as u64;
    skip(cursor, 4);

    let mut hm: HashMap<String, String> = HashMap::new();
    while position < end {
        let k1 = read_matrix_string(cursor);
        let k2 = read_matrix_string(cursor);
        let k3 = read_matrix_string(cursor);
        let len_inner = read_u32_le(cursor);
        for _ in 0..len_inner {
            let k4 = read_matrix_string(cursor);
            let v = read_matrix_string(cursor);
            hm.insert(format!("{}::{}::{}.{}", k1, k2, k3, k4), v);
        }
        position = cursor.position();
    }
    IdentBlock::INST(hm)
}

// CNXS
// this block is nested third in EXPS
fn read_cnxs(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let len = read_u32_le(cursor);

    let mut position = cursor.position();
    let end = position + len as u64;
    skip(cursor, 4);

    let mut hm: HashMap<String, String> = HashMap::new();
    while position < end {
        let k1 = read_matrix_string(cursor);
        let k2 = read_matrix_string(cursor);
        let len_inner = read_u32_le(cursor);
        for _ in 0..len_inner {
            let k3 = read_matrix_string(cursor);
            let v = read_matrix_string(cursor);
            hm.insert(format!("{}::{}.{}", k1, k2, k3), v);
        }
        position = cursor.position();
    }
    IdentBlock::CNXS(hm)
}

// EEPA
fn read_eepa(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unused = read_u32_le(cursor);

    skip(cursor, 4);

    let len_outer = read_u32_le(cursor);

    let mut hm: HashMap<String, MatrixType> = HashMap::new();
    for _ in 0..len_outer {
        let inst = read_matrix_string(cursor);
        let len_inner = read_u32_le(cursor);
        for _ in 0..len_inner {
            let prop = read_matrix_string(cursor);
            let unit = read_matrix_string(cursor);
            // don't know if this is useful
            let _empty = read_u32_le(cursor);
            let matrix_type: String = read_matrix_type(cursor);
            let value = match matrix_type.as_str() {
                "BOOL" => MatrixType::BOOL(read_u32_le(cursor)),
                "LONG" => MatrixType::LONG(read_u32_le(cursor)),
                "STRG" => MatrixType::STRG(read_matrix_string(cursor)),
                "DOUB" => MatrixType::DOUB(read_f64_le(cursor)),
                _ => unimplemented!(),
            };
            hm.insert(format!("{}.{} [{}]", inst, prop, unit), value);
        }
    }
    IdentBlock::EEPA(hm)
}

// INCI
// state of experiment
fn read_inci(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unused = read_u32_le(cursor);

    let _ = read_u32_le(cursor);
    let _ = read_u32_le(cursor);
    IdentBlock::INCI("".to_string())
}

// MARK
// calibration of system
fn read_mark(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unused = read_u32_le(cursor);

    let content = read_matrix_string(cursor);
    IdentBlock::MARK(content)
}

// VIEW
// scanning windows settings
fn read_view(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unused = read_u32_le(cursor);

    let content = read_string(cursor, len as usize);
    IdentBlock::VIEW(content)
}

// PROC
// processors of scanning windows (plugins, e.g. CurveAverager, Despiker)
fn read_proc(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unused = read_u32_le(cursor);

    let content = read_string(cursor, len as usize);
    IdentBlock::PROC(content)
}

// PMOD
fn read_pmod(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unused = read_u32_le(cursor);
    skip(cursor, 4);

    let category = read_matrix_string(cursor);
    let prop = read_matrix_string(cursor);
    let unit = read_matrix_string(cursor);

    skip(cursor, 4);
    let matrix_type = read_matrix_type(cursor);

    let value = match matrix_type.as_str() {
        "BOOL" => MatrixType::BOOL(read_u32_le(cursor)),
        "LONG" => MatrixType::LONG(read_u32_le(cursor)),
        "STRG" => MatrixType::STRG(read_matrix_string(cursor)),
        "DOUB" => MatrixType::DOUB(read_f64_le(cursor)),
        _ => unreachable!(),
    };
    skip(cursor, 4);

    let mut hm: HashMap<String, MatrixType> = HashMap::new();
    hm.insert(format!("{}.{} [{}]", category, prop, unit), value);
    IdentBlock::PMOD(hm)
}

// CCSY
// has nested blocks DICT, CHCS, SCAN, XFER
fn read_ccsy(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    // println!("CCSY len: {}", len);
    let _time = read_u32_le(cursor);
    let _unused = read_u32_le(cursor);

    skip(cursor, 4);

    // let inner_block = read_matrix_type(cursor);
    // println!("inner: {}", inner_block);
    // skip(cursor, len as u64);

    IdentBlock::CCSY("".to_string())
}

// DICT
// nested in CCSY
fn read_dict(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    // println!("DICT len: {}", _len);

    let _ = read_u32_le(cursor); // no time in here
    let _unused = read_u32_le(cursor);

    let n_first = read_u32_le(cursor);
    for _ in 0..n_first {
        skip(cursor, 16); // could also be 4 different u32
        let _s1 = read_matrix_string(cursor);
        let _s2 = read_matrix_string(cursor);
    }

    let n_second = read_u32_le(cursor);
    let mut hm: HashMap<String, u32> = HashMap::new();
    for _ in 0..n_second {
        skip(cursor, 4);
        // This seems to be some info about channels
        let channel_num = read_u32_le(cursor);
        skip(cursor, 8);

        let channel = read_matrix_string(cursor);
        let unit = read_matrix_string(cursor);
        hm.insert(format!("{} [{}]", channel, unit), channel_num);
    }

    let n_third = read_u32_le(cursor);
    for _ in 0..n_third {
        skip(cursor, 16); // could be 4 times u32
        let _s3 = read_matrix_string(cursor);
    }
    IdentBlock::DICT(hm)
}

// CHCS
// netsted in CCSY
fn read_chcs(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    // println!("CHCS len: {}", _len);

    let n_first = read_u32_le(cursor);
    // println!("n_first: {}", n_first);
    for _ in 0..n_first {
        let _a = read_u32_le(cursor);
        // println!("a: {}", a);
        let _b = read_u32_le(cursor);
        // println!("b: {}", b);
        let _c = read_u32_le(cursor);
        // println!("c: {}", c);
        let _d = read_u32_le(cursor);
        // println!("d: {}", d);
        let _e = read_u32_le(cursor);
        // println!("e: {}", e);
    }

    let n_second = read_u32_le(cursor);
    // println!("n_sec: {}", n_second);
    for _ in 0..n_second {
        let _f = read_u32_le(cursor);
        // println!("f: {}", f);
        let _g = read_u32_le(cursor);
        // println!("g: {}", g);
        let _h = read_u32_le(cursor);
        // println!("h: {}", h);
        let _i = read_u32_le(cursor);
        // println!("i: {}", i);
    }

    let n_third = read_u32_le(cursor);
    // println!("n_third: {}", n_third);
    for _ in 0..n_third {
        let _j = read_u32_le(cursor);
        // println!("j: {}", j);
        let _k = read_u32_le(cursor);
        // println!("k: {}", k);
        let _l = read_u32_le(cursor);
        // println!("l: {}", l);
        let _m = read_u32_le(cursor);
        // println!("m: {}", m);
    }
    IdentBlock::CHCS("".to_string())
}

// SCAN
// netsted in CCSY
fn read_scan(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    // println!("SCAN len: {}", _len);
    //
    // let n = read_u32_le(cursor);  // no time in here
    // println!("SCAN n: {}", n);
    // for _ in 0..n {
    // }

    skip(cursor, _len as u64);
    IdentBlock::SCAN("".to_string())
}

// XFER
// netsted in CCSY
fn read_xfer(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    // println!("XFER len: {}", _len);

    let mut position = cursor.position();
    let end = position + _len as u64;

    let mut hm: HashMap<String, MatrixType> = HashMap::new();
    while position < end {
        skip(cursor, 4);
        let _n = read_u32_le(cursor);
        // println!("n: {}", _n);
        let name = read_matrix_string(cursor);
        let unit = read_matrix_string(cursor);

        let len_inner = read_u32_le(cursor);
        for _ in 0..len_inner {
            let prop = read_matrix_string(cursor);
            let matrix_type = read_matrix_type(cursor);
            let value = match matrix_type.as_str() {
                "BOOL" => MatrixType::BOOL(read_u32_le(cursor)),
                "LONG" => MatrixType::LONG(read_u32_le(cursor)),
                "STRG" => MatrixType::STRG(read_matrix_string(cursor)),
                "DOUB" => MatrixType::DOUB(read_f64_le(cursor)),
                _ => unreachable!(),
            };
            hm.insert(format!("{}.{} [{}]", name, prop, unit), value);
        }
        position = cursor.position();
    }
    println!("XFER hm: {:#?}", hm);
    IdentBlock::XFER(hm)
}

// BREF
fn read_bref(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unbytes = read_u32_le(cursor);

    skip(cursor, 4);

    let filename = read_matrix_string(cursor);
    IdentBlock::BREF(filename)
}

// EOED
fn read_eoed(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let _len = read_u32_le(cursor);
    let _time = read_u32_le(cursor);
    let _unbytes = read_u32_le(cursor);
    println!("END OF FILE");

    IdentBlock::EOED(true)
}
