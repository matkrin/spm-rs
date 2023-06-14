use std::collections::HashMap;
use std::fs::read;
use std::io::Cursor;
use std::str;

use crate::utils::Bytereading;

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

// returns a Vec of all IdentBlock in the paramfile
pub fn _read_omicron_matrix_paramfile_full(filename: &str) -> Vec<IdentBlock> {
    let bytes = read(filename).unwrap();
    let mut cursor = Cursor::new(bytes.as_slice());
    let magic_header = cursor.read_magic_header();
    assert_eq!(magic_header, "ONTMATRX0101");

    let file_length = bytes.len();
    let mut position = 0;
    let mut v = Vec::new();
    while position < file_length as u64 {
        v.push(read_ident_block(&mut cursor));
        position = cursor.position();
    }
    assert_eq!(cursor.position(), file_length as u64);
    v
}

pub fn read_ident_block(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let ident: String = cursor.read_matrix_type();

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
fn read_meta(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unused = cursor.read_u32_le();

    let mut hm: HashMap<String, String> = HashMap::new();
    hm.insert("program_name".to_string(), cursor.read_matrix_string());
    hm.insert("version".to_string(), cursor.read_matrix_string());

    cursor.skip(4);

    hm.insert("profile".to_string(), cursor.read_matrix_string());
    hm.insert("user".to_string(), cursor.read_matrix_string());

    cursor.skip(4);

    IdentBlock::META(hm)
}

//EXPD
fn read_expd(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unused = cursor.read_u32_le();

    cursor.skip(4);

    let mut content = String::new();
    for _ in 0..7 {
        let s = cursor.read_matrix_string();
        content += &format!("\n{}", s);
    }
    IdentBlock::EXPD(content.trim().to_owned())
}

// FSEQ
fn read_fseq(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    // let len = read_u32_le(cursor);
    // let time = read_u32_le(cursor);
    cursor.skip(20);
    IdentBlock::FSEQ("".to_string())
}

// EXPS contains INST and CNXS
// therefore len of exps contains len of inst and CNXS
// reading of those blocks is also handled by read_ident_block
fn read_exps(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unused = cursor.read_u32_le();
    cursor.skip(4);

    IdentBlock::EXPS("".to_string())
}

// GENL
// this block is nested first in EXPS
fn read_genl(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();

    let a = cursor.read_matrix_string();
    let b = cursor.read_matrix_string();
    let c = cursor.read_matrix_string();
    IdentBlock::GENL(format!("{}; {}; {}", a, b, c))
}

// INST
// this block is nested second in EXPS
fn read_inst(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let len = cursor.read_u32_le();

    let mut position = cursor.position();
    let end = position + len as u64;
    cursor.skip(4);

    let mut hm: HashMap<String, String> = HashMap::new();
    while position < end {
        let k1 = cursor.read_matrix_string();
        let k2 = cursor.read_matrix_string();
        let k3 = cursor.read_matrix_string();
        let len_inner = cursor.read_u32_le();
        for _ in 0..len_inner {
            let k4 = cursor.read_matrix_string();
            let v = cursor.read_matrix_string();
            hm.insert(format!("{}::{}::{}.{}", k1, k2, k3, k4), v);
        }
        position = cursor.position();
    }
    IdentBlock::INST(hm)
}

// CNXS
// this block is nested third in EXPS
fn read_cnxs(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let len = cursor.read_u32_le();

    let mut position = cursor.position();
    let end = position + len as u64;
    cursor.skip(4);

    let mut hm: HashMap<String, String> = HashMap::new();
    while position < end {
        let k1 = cursor.read_matrix_string();
        let k2 = cursor.read_matrix_string();
        let len_inner = cursor.read_u32_le();
        for _ in 0..len_inner {
            let k3 = cursor.read_matrix_string();
            let v = cursor.read_matrix_string();
            hm.insert(format!("{}::{}.{}", k1, k2, k3), v);
        }
        position = cursor.position();
    }
    IdentBlock::CNXS(hm)
}

// EEPA
fn read_eepa(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unused = cursor.read_u32_le();

    cursor.skip(4);

    let len_outer = cursor.read_u32_le();

    let mut hm: HashMap<String, MatrixType> = HashMap::new();
    for _ in 0..len_outer {
        let inst = cursor.read_matrix_string();
        let len_inner = cursor.read_u32_le();
        for _ in 0..len_inner {
            let prop = cursor.read_matrix_string();
            let unit = cursor.read_matrix_string();
            // don't know if this is useful
            let _empty = cursor.read_u32_le();
            let matrix_type: String = cursor.read_matrix_type();
            let value = match matrix_type.as_str() {
                "BOOL" => MatrixType::BOOL(cursor.read_u32_le()),
                "LONG" => MatrixType::LONG(cursor.read_u32_le()),
                "STRG" => MatrixType::STRG(cursor.read_matrix_string()),
                "DOUB" => MatrixType::DOUB(cursor.read_f64_le()),
                _ => unimplemented!(),
            };
            hm.insert(format!("{}.{} [{}]", inst, prop, unit), value);
        }
    }
    IdentBlock::EEPA(hm)
}

// INCI
// state of experiment
fn read_inci(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unused = cursor.read_u32_le();

    let _ = cursor.read_u32_le();
    let _ = cursor.read_u32_le();
    IdentBlock::INCI("".to_string())
}

// MARK
// calibration of system
fn read_mark(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unused = cursor.read_u32_le();

    let content = cursor.read_matrix_string();
    IdentBlock::MARK(content)
}

// VIEW
// scanning windows settings
fn read_view(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unused = cursor.read_u32_le();

    let content = cursor.read_string(len as usize);
    IdentBlock::VIEW(content)
}

// PROC
// processors of scanning windows (plugins, e.g. CurveAverager, Despiker)
fn read_proc(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unused = cursor.read_u32_le();

    let content = cursor.read_string(len as usize);
    IdentBlock::PROC(content)
}

// PMOD
fn read_pmod(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unused = cursor.read_u32_le();
    cursor.skip(4);

    let category = cursor.read_matrix_string();
    let prop = cursor.read_matrix_string();
    let unit = cursor.read_matrix_string();

    cursor.skip(4);
    let matrix_type = cursor.read_matrix_type();

    let value = match matrix_type.as_str() {
        "BOOL" => MatrixType::BOOL(cursor.read_u32_le()),
        "LONG" => MatrixType::LONG(cursor.read_u32_le()),
        "STRG" => MatrixType::STRG(cursor.read_matrix_string()),
        "DOUB" => MatrixType::DOUB(cursor.read_f64_le()),
        _ => unreachable!(),
    };
    cursor.skip(4);

    let mut hm: HashMap<String, MatrixType> = HashMap::new();
    hm.insert(format!("{}.{} [{}]", category, prop, unit), value);
    IdentBlock::PMOD(hm)
}

// CCSY
// has nested blocks DICT, CHCS, SCAN, XFER
fn read_ccsy(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unused = cursor.read_u32_le();

    cursor.skip(4);
    IdentBlock::CCSY("".to_string())
}

// DICT
// nested in CCSY
fn read_dict(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _ = cursor.read_u32_le(); // no time in here
    let _unused = cursor.read_u32_le();

    let n_first = cursor.read_u32_le();
    for _ in 0..n_first {
        cursor.skip(16); // could also be 4 different u32
        let _s1 = cursor.read_matrix_string();
        let _s2 = cursor.read_matrix_string();
    }

    let n_second = cursor.read_u32_le();
    let mut hm: HashMap<String, u32> = HashMap::new();
    for _ in 0..n_second {
        cursor.skip(4);
        // This seems to be some info about channels
        let channel_num = cursor.read_u32_le();
        cursor.skip(8);

        let channel = cursor.read_matrix_string();
        let unit = cursor.read_matrix_string();
        hm.insert(format!("{} [{}]", channel, unit), channel_num);
    }

    let n_third = cursor.read_u32_le();
    for _ in 0..n_third {
        cursor.skip(16); // could be 4 times u32
        let _s3 = cursor.read_matrix_string();
    }
    IdentBlock::DICT(hm)
}

// CHCS
// netsted in CCSY
fn read_chcs(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    // println!("CHCS len: {}", _len);

    let n_first = cursor.read_u32_le();
    // println!("n_first: {}", n_first);
    for _ in 0..n_first {
        let _a = cursor.read_u32_le();
        // println!("a: {}", a);
        let _b = cursor.read_u32_le();
        // println!("b: {}", b);
        let _c = cursor.read_u32_le();
        // println!("c: {}", c);
        let _d = cursor.read_u32_le();
        // println!("d: {}", d);
        let _e = cursor.read_u32_le();
        // println!("e: {}", e);
    }

    let n_second = cursor.read_u32_le();
    // println!("n_sec: {}", n_second);
    for _ in 0..n_second {
        let _f = cursor.read_u32_le();
        // println!("f: {}", f);
        let _g = cursor.read_u32_le();
        // println!("g: {}", g);
        let _h = cursor.read_u32_le();
        // println!("h: {}", h);
        let _i = cursor.read_u32_le();
        // println!("i: {}", i);
    }

    let n_third = cursor.read_u32_le();
    // println!("n_third: {}", n_third);
    for _ in 0..n_third {
        let _j = cursor.read_u32_le();
        // println!("j: {}", j);
        let _k = cursor.read_u32_le();
        // println!("k: {}", k);
        let _l = cursor.read_u32_le();
        // println!("l: {}", l);
        let _m = cursor.read_u32_le();
        // println!("m: {}", m);
    }
    IdentBlock::CHCS("".to_string())
}

// SCAN
// netsted in CCSY
fn read_scan(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    // println!("SCAN len: {}", _len);
    //
    // let n = read_u32_le(cursor);  // no time in here
    // println!("SCAN n: {}", n);
    // for _ in 0..n {
    // }

    cursor.skip(_len as u64);
    IdentBlock::SCAN("".to_string())
}

// XFER
// netsted in CCSY
fn read_xfer(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();

    let mut position = cursor.position();
    let end = position + _len as u64;

    let mut hm: HashMap<String, MatrixType> = HashMap::new();
    while position < end {
        cursor.skip(4);
        let _n = cursor.read_u32_le();
        let name = cursor.read_matrix_string();
        let unit = cursor.read_matrix_string();

        let len_inner = cursor.read_u32_le();
        for _ in 0..len_inner {
            let prop = cursor.read_matrix_string();
            let matrix_type = cursor.read_matrix_type();
            let value = match matrix_type.as_str() {
                "BOOL" => MatrixType::BOOL(cursor.read_u32_le()),
                "LONG" => MatrixType::LONG(cursor.read_u32_le()),
                "STRG" => MatrixType::STRG(cursor.read_matrix_string()),
                "DOUB" => MatrixType::DOUB(cursor.read_f64_le()),
                _ => unreachable!(),
            };
            hm.insert(format!("{}.{} [{}]", name, prop, unit), value);
        }
        position = cursor.position();
    }
    IdentBlock::XFER(hm)
}

// BREF
fn read_bref(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unbytes = cursor.read_u32_le();

    cursor.skip(4);

    let filename = cursor.read_matrix_string();
    IdentBlock::BREF(filename)
}

// EOED
fn read_eoed(cursor: &mut Cursor<&[u8]>) -> IdentBlock {
    let _len = cursor.read_u32_le();
    let _time = cursor.read_u32_le();
    let _unbytes = cursor.read_u32_le();
    IdentBlock::EOED(true)
}
