// use mulfile_rs::read_mul;
use mulfile_rs::read_omicron_matrix_scanfile;
use mulfile_rs::{read_omicron_matrix_paramfile, IdentBlock, MatrixType};

fn main() {
    // let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    //
    // for i in mulfile {
    //     i.correct_plane().correct_lines().save_png();
    // }

    let filename = "20201111--4_1.Z_mtrx";
    let p = read_omicron_matrix_paramfile("20201111_0001.mtrx");
    let s = read_omicron_matrix_scanfile(filename);

    // Keys
    static CURRENT: &'static str = "Regulator.Setpoint_1 [Ampere]";
    static CURRENT_ALT: &'static str = "Regulator.Alternate_Setpoint_1 [Ampere]";
    static BIAS: &'static str = "GapVoltageControl.Voltage [Volt]";
    static XSIZE: &'static str = "XYScanner.Width [Meter]";
    static YSIZE: &'static str = "XYScanner.Height [Meter]";
    static XRES: &'static str = "XYScanner.Points [Count]";
    static YRES: &'static str = "XYScanner.Lines [Count]";
    static ROTATION: &'static str = "XYScanner.Angle [Degree]";
    static RASTER_TIME: &'static str = "XYScanner.Raster_Time [Second]";
    static XOFFSET: &'static str = "XYScanner.X_Offset [Meter]";
    static YOFFSET: &'static str = "XYScanner.Y_Offset [Meter]";

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
    // 1. read EEPA which gives initial values, in EEPA all keys should be in one Hashmap
    // 2. change initial values if the PMOD with the key for this value appears
    // 3. break if BREF with filename to look appears
    // => therefore values are always the ones from last PMOD, which should be right

    for i in p {
        match i {
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
                }
            }
            IdentBlock::BREF(x) => {
                println!("f: {}", x);
                if x == filename {
                    break;
                }
            }
            _ => continue,
        };
    }

    println!("current: {}", current * 1e9);
    println!("bias: {}", bias);
    println!("xsize: {}", xsize * 1e9);
    println!("ysize: {}", ysize * 1e9);
    println!("xres: {}", xres);
    println!("yres: {}", yres);
    println!("rotation: {}", rotation);
    println!("raster time: {}", raster_time);
    println!("xoffset: {}", xoffset);
    println!("yoffset: {}", yoffset);
}
