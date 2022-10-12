use std::collections::HashMap;
use std::fs::read;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::str;

pub fn read_omicron_matrix_paramfile(filename: &str) {
    let bytes = read(filename).unwrap();
    let mut cursor = Cursor::new(&bytes);
    let magic_header = read_magic_header(&mut cursor);
    assert_eq!(magic_header, "ONTMATRX0101");

    let file_length = bytes.len();
    let mut position = 0;
    while position < file_length as u64 {
        let i = read_ident_block(&mut cursor);
        println!("i: {:?}", i);
        position = cursor.position();
    }
}

#[derive(Debug)]
enum IdentBlock {
    META(META),
    EXPD(String),
    FSEQ(String),
    EXPS(String),
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
    _GENL(String),
}

#[derive(Debug)]
struct META {
    program_name: String,
    version: String,
    profile: String,
    user: String,
}

#[derive(Debug)]
enum MatrixType {
    BOOL(u32),
    LONG(u32),
    STRG(String),
    DOUB(f64),
}

fn read_ident_block(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    let ident: String = read_string(cursor, 4).chars().rev().collect();
    // println!("ident: {}", ident);

    match ident.as_str() {
        "META" => read_meta(cursor),
        "EXPD" => read_expd(cursor),
        "FSEQ" => read_fseq(cursor),
        "EXPS" => read_exps(cursor),
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

    let program_name = read_matrix_string(cursor);
    let version = read_matrix_string(cursor);

    let _ = read_u32_le(cursor);
    let profile = read_matrix_string(cursor);
    let user = read_matrix_string(cursor);

    let _ = read_u32_le(cursor);

    let meta = META {
        program_name,
        version,
        profile,
        user,
    };
    println!("meta: {:?}", meta);
    IdentBlock::META(meta)
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
    skip(cursor, 8);

    // not sure what this is good for
    let _a = read_u32_le(cursor);
    // println!("a: {}", a);
    let b = read_matrix_string(cursor);
    let c = read_matrix_string(cursor);
    let d = read_matrix_string(cursor);

    IdentBlock::EXPS(format!("{}; {}; {}", b, c, d))
}

// INST
// this block is contained in EXPS
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
// this block is contained in EXPS
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
    let len = read_u32_le(cursor);
    println!("CCSY len: {}", len);
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
    println!("DICT len: {}", _len);

    let _ = read_u32_le(cursor);  // no time in here
    let _unused = read_u32_le(cursor);

    let n_first = read_u32_le(cursor);
    for _ in 0..n_first {
        skip(cursor, 16);  // could also be 4 different u32
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
        skip(cursor, 16);  // could be 4 times u32
        let _s3 = read_matrix_string(cursor);
        
    }
    IdentBlock::DICT(hm)
}

// CHCS
// netsted in CCSY
fn read_chcs(cursor: &mut Cursor<&Vec<u8>>) -> IdentBlock {
    println!("positin start CHCS: {}", cursor.position());
    let _len = read_u32_le(cursor);
    println!("CHCS len: {}", _len);

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
    println!("position end CHCS: {}", cursor.position());
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
    println!("XFER len: {}", _len);

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
    println!("End of file");

    IdentBlock::EOED(true)
}

//-------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------
fn skip(cursor: &mut Cursor<&Vec<u8>>, num_bytes: u64) {
    cursor.set_position(cursor.position() + num_bytes);
}

fn read_matrix_type(cursor: &mut Cursor<&Vec<u8>>) -> String {
    read_string(cursor, 4).chars().rev().collect()
}

fn read_matrix_string(cursor: &mut Cursor<&Vec<u8>>) -> String {
    let string_length = read_u32_le(cursor);
    // println!{"string length: {}", string_length};
    read_utf16_string(cursor, string_length as usize)
}

fn read_utf16_string(cursor: &mut Cursor<&Vec<u8>>, length: usize) -> String {
    let mut buffer = vec![0; length * 2];
    cursor.read_exact(&mut buffer).expect("to read");
    read_utf16_bytes(&buffer)
}

fn read_utf16_bytes(slice: &[u8]) -> String {
    // assert!(2*size <= slice.len());
    // println!("size: {}", size);
    // println!("len: {}", slice.len());
    let iter = (0..(slice.len() / 2)).map(|i| u16::from_le_bytes([slice[2 * i], slice[2 * i + 1]]));

    let result = std::char::decode_utf16(iter)
        .collect::<Result<String, _>>()
        .unwrap();
    result
}

fn read_mul_str(buffer: &[u8]) -> &str {
    str::from_utf8(buffer).expect("to read_str")
}

fn read_magic_header(cursor: &mut Cursor<&Vec<u8>>) -> String {
    let mut buffer = [0; 12];
    cursor.read_exact(&mut buffer).unwrap();
    read_mul_str(&buffer).to_owned()
}

fn read_string(cursor: &mut Cursor<&Vec<u8>>, length: usize) -> String {
    let mut buffer = vec![0; length];
    cursor.read_exact(&mut buffer).expect("to read");
    read_mul_str(&buffer).to_owned()
}

fn read_u32_le_bytes(buffer: &[u8]) -> u32 {
    u32::from_le_bytes(buffer[..4].try_into().unwrap())
}

fn read_u32_le(cursor: &mut Cursor<&Vec<u8>>) -> u32 {
    let mut buffer = [0; 4];
    cursor.read_exact(&mut buffer).unwrap();
    read_u32_le_bytes(&buffer)
}

fn read_u64_le_bytes(buffer: &[u8]) -> u64 {
    u64::from_le_bytes(buffer[..8].try_into().unwrap())
}

fn read_u64_le(cursor: &mut Cursor<&Vec<u8>>) -> u64 {
    let mut buffer = [0; 8];
    cursor.read_exact(&mut buffer).unwrap();
    read_u64_le_bytes(&buffer)
}

fn read_f64_le_bytes(buffer: &[u8]) -> f64 {
    f64::from_le_bytes(buffer[..8].try_into().unwrap())
}

fn read_f64_le(cursor: &mut Cursor<&Vec<u8>>) -> f64 {
    let mut buffer = [0; 8];
    cursor.read_exact(&mut buffer).unwrap();
    read_f64_le_bytes(&buffer)
}
