use std::collections::HashMap;
use std::fs::read;
use std::io::Cursor;
use std::path::Path;

use anyhow::Result;

use crate::omicron_matrix::paramfile::{read_ident_block, IdentBlock, MatrixType};
use crate::utils::Bytereading;

#[derive(Debug)]
pub struct ParamData {
    pub current: f64,
    pub bias: f64,
    pub xsize: f64,
    pub ysize: f64,
    pub xres: u32,
    pub yres: u32,
    pub rotation: u32,
    pub raster_time: f64,
    pub xoffset: f64,
    pub yoffset: f64,
    pub xretrace: bool,
    pub yretrace: bool,
    pub tffs: HashMap<String, f64>,
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

pub fn get_param_info(filename: &str) -> Result<ParamData> {
    let basename = Path::new(filename).file_name().unwrap().to_str().unwrap();
    let paramfile = format!("{}_0001.mtrx", filename.split_once("--").unwrap().0);
    let bytes = read(paramfile).unwrap();
    let mut cursor = Cursor::new(bytes.as_slice());
    let magic_header = cursor.read_magic_header();
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
    let mut tffs: HashMap<String, f64> = HashMap::new();

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
            IdentBlock::XFER(hm) => {
                for (key, value) in hm.iter() {
                    if let MatrixType::DOUB(x) = value {
                        tffs.insert(key.to_owned(), *x);
                    }
                }
            }
            IdentBlock::BREF(x) => {
                if x == basename {
                    break;
                }
            }
            _ => continue,
        };
        position = cursor.position();
    }

    Ok(ParamData {
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
        tffs,
    })
}
