use std::fs::read;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::str;
use std::path::PathBuf;
use chrono::prelude::*;
use chrono::{DateTime, Utc};

// i16
fn read_i16_le_bytes(buffer: &[u8]) -> i16 {
    i16::from_le_bytes(buffer[..2].try_into().unwrap())
}

fn read_i16_le(cursor: &mut Cursor<&Vec<u8>>) -> i16 {
    let mut buffer = [0; 2];
    cursor.read_exact(&mut buffer).unwrap();
    read_i16_le_bytes(&buffer)
}

// i32
fn read_i32_le_bytes(buffer: &[u8]) -> i32 {
    i32::from_le_bytes(buffer[..4].try_into().unwrap())
}

fn read_i32_le(cursor: &mut Cursor<&Vec<u8>>) -> i32 {
    let mut buffer = [0; 4];
    cursor.read_exact(&mut buffer).unwrap();
    read_i32_le_bytes(&buffer)
}

// string
fn read_str(buffer: &[u8]) -> &str {
    str::from_utf8(&buffer[..21]).unwrap()
}

fn read_string(cursor: &mut Cursor<&Vec<u8>>) -> String {
    let mut buffer = [0; 21];
    cursor.read_exact(&mut buffer).unwrap();
    read_str(&buffer).to_owned()
}

// Image Data
fn read_pixels(buffer: &[u8], zscale: i32) -> Vec<f64> {
    let mut pixels: Vec<f64> = Vec::with_capacity(buffer.len() / 2);
    let mut i = 0;
    while i < buffer.len() {
        let pixel = f64::from(read_i16_le_bytes(&buffer[i..i + 2])) * -0.1 / 1.36
            * f64::from(zscale)
            / 2000.0;
        pixels.push(pixel);
        i += 2;
    }
    pixels
}

fn read_img_data(cursor: &mut Cursor<&Vec<u8>>, num_pixels: i32, zscale: i32) -> Vec<f64> {
    let mut buffer = vec![0; (num_pixels * 2) as usize];
    cursor.read_exact(&mut buffer).unwrap();
    read_pixels(&buffer, zscale)
}

// Point Scan Data
fn read_data_points(buffer: &[u8]) -> Vec<f64> {
    let mut data_points: Vec<f64> = Vec::with_capacity(buffer.len() / 2);
    let mut i = 0;
    while i < buffer.len() {
        let data_point = f64::from(read_i16_le_bytes(&buffer[i..i + 2]));
        data_points.push(data_point);
        i += 2;
    }
    data_points
}

fn read_point_scan(cursor: &mut Cursor<&Vec<u8>>, num_data_points: i32) -> Vec<f64> {
    let mut buffer = vec![0; (num_data_points * 2) as usize];
    cursor.read_exact(&mut buffer).unwrap();
    read_data_points(&buffer)
}

#[derive(Debug)]
struct MulImage {
    filepath: PathBuf,
    img_num: i32,
    img_id: String,
    size: i32,
    xres: i32,
    yres: i32,
    zres: i32,
    datetime: DateTime<Utc>,
    xsize: i32,
    ysize: i32,
    xoffset: i32,
    yoffset: i32,
    zscale: i32,
    tilt: i32,
    speed: f64,
    line_time: f64,
    bias: f64,
    current: f64,
    sample: String,
    title: String,
    postpr: i32,
    postd1: i32,
    mode: i32,
    currfac: i32,
    num_pointscans: i32,
    unitnr: i32,
    version: i32,
    gain: i32,
    img_data: Vec<f64>,
}

fn read_mul(filename: &str) -> Vec<MulImage> {
    const MUL_BLOCK: i32 = 128;
    let mut block_counter = 0;
    let mut mul: Vec<MulImage> = Vec::new();

    let bytes = read(filename).unwrap();
    let mut cursor = Cursor::new(&bytes);

    let nr = read_i16_le(&mut cursor);
    let adr = read_i32_le(&mut cursor);


    if adr == 3 {
        cursor
            .seek(SeekFrom::Start((adr * MUL_BLOCK) as u64))
            .expect("seeking failed");
        block_counter += adr;
    }

    while block_counter * MUL_BLOCK < bytes.len().try_into().unwrap() {
        let img_num = read_i16_le(&mut cursor);
        let size = read_i16_le(&mut cursor);

        let xres = read_i16_le(&mut cursor);
        let yres = read_i16_le(&mut cursor);
        let zres = read_i16_le(&mut cursor);

        let year = read_i16_le(&mut cursor);
        let month = read_i16_le(&mut cursor);
        let day = read_i16_le(&mut cursor);
        let hour = read_i16_le(&mut cursor);
        let minute = read_i16_le(&mut cursor);
        let second = read_i16_le(&mut cursor);

        let xsize = read_i16_le(&mut cursor) / 10; // in nm
        let ysize = read_i16_le(&mut cursor) / 10; // in nm

        let xoffset = read_i16_le(&mut cursor) / 10; // in nm
        let yoffset = read_i16_le(&mut cursor) / 10; // in nm

        let zscale = read_i16_le(&mut cursor);
        let tilt = read_i16_le(&mut cursor);
        let speed = read_i16_le(&mut cursor) / 100; // in s

        let bias = read_i16_le(&mut cursor);
        let current = read_i16_le(&mut cursor);

        let sample = read_string(&mut cursor);
        let title = read_string(&mut cursor);

        let postpr = read_i16_le(&mut cursor);
        let postd1 = read_i16_le(&mut cursor);
        let mode = read_i16_le(&mut cursor);
        let currfac = read_i16_le(&mut cursor);
        let num_pointscans = read_i16_le(&mut cursor);
        let unitnr = read_i16_le(&mut cursor);
        let version = read_i16_le(&mut cursor);

        let spare_48 = read_i16_le(&mut cursor);
        let spare_49 = read_i16_le(&mut cursor);
        let spare_50 = read_i16_le(&mut cursor);
        let spare_51 = read_i16_le(&mut cursor);
        let spare_52 = read_i16_le(&mut cursor);
        let spare_53 = read_i16_le(&mut cursor);
        let spare_54 = read_i16_le(&mut cursor);
        let spare_54 = read_i16_le(&mut cursor);
        let spare_56 = read_i16_le(&mut cursor);
        let spare_57 = read_i16_le(&mut cursor);
        let spare_58 = read_i16_le(&mut cursor);
        let spare_59 = read_i16_le(&mut cursor);

        let gain = read_i16_le(&mut cursor);

        let spare_61 = read_i16_le(&mut cursor);
        let spare_62 = read_i16_le(&mut cursor);
        let spare_63 = read_i16_le(&mut cursor);

        let img_data = read_img_data(
            &mut cursor,
            (xres as i32 * yres as i32).into(),
            zscale.into(),
        );
        // println!("img data: {:#?}", img_data);
        // println!("img data first: {}", img_data[0]);
        // println!("len img data: {:#?}", img_data.len());

        if num_pointscans > 0 {
            for _ in 0..num_pointscans {
                let ps_size = read_i16_le(&mut cursor);
                let ps_type = read_i16_le(&mut cursor);
                let ps_time4scan = read_i16_le(&mut cursor);
                let ps_minv = read_i16_le(&mut cursor);
                let ps_maxv = read_i16_le(&mut cursor);
                let ps_xpos = read_i16_le(&mut cursor);
                let ps_ypos = read_i16_le(&mut cursor);
                let ps_dz = read_i16_le(&mut cursor);
                let ps_delay = read_i16_le(&mut cursor);
                let ps_version = read_i16_le(&mut cursor);
                let ps_indendelay = read_i16_le(&mut cursor);
                let ps_xposend = read_i16_le(&mut cursor);
                let ps_yposend = read_i16_le(&mut cursor);
                let ps_vt_fw = read_i16_le(&mut cursor);
                let ps_it_fw = read_i16_le(&mut cursor);
                let ps_vt_bw = read_i16_le(&mut cursor);
                let ps_it_bw = read_i16_le(&mut cursor);
                let ps_lscan = read_i16_le(&mut cursor);

                cursor.seek(SeekFrom::Current((MUL_BLOCK - 18 * 2) as i64));

                let ps_data = read_point_scan(&mut cursor, ps_size as i32);
            }
        }

        let line_time = f64::from(speed) / f64::from(yres) * 1000.0; // in ms
        let bias = -f64::from(bias) / 3.2768; //  in mV
        let current = f64::from(current) * f64::from(currfac) * 0.01; // in nA

        let datetime = Utc
            .ymd(
                year.try_into().unwrap(),
                month.try_into().unwrap(),
                day.try_into().unwrap(),
            )
            .and_hms(
                hour.try_into().unwrap(),
                minute.try_into().unwrap(),
                second.try_into().unwrap(),
            );

        let filepath = PathBuf::from(&filename);
        let basename = filepath.file_name().unwrap().to_str().unwrap();
        let img_id = format!("{}_{}", basename, img_num);

        block_counter += size as i32;

        mul.push(MulImage {
            filepath,
            img_num: img_num.into(),
            img_id: img_id.into(),
            size: size.into(),
            xres: xres.into(),
            yres: yres.into(),
            zres: zres.into(),
            datetime,
            xsize: xsize.into(),
            ysize: ysize.into(),
            xoffset: xoffset.into(),
            yoffset: yoffset.into(),
            zscale: zscale.into(),
            tilt: tilt.into(),
            speed: speed.into(),
            line_time,
            bias,
            current,
            sample,
            title,
            postpr: postpr.into(),
            postd1: postd1.into(),
            mode: mode.into(),
            currfac: currfac.into(),
            num_pointscans: num_pointscans.into(),
            unitnr: unitnr.into(),
            version: version.into(),
            gain: gain.into(),
            img_data,
        })
    }
    assert_eq!{block_counter * MUL_BLOCK, bytes.len() as i32};
    mul
}

fn main() {
    let all = read_mul("stm-aarhus-mul-a.mul");
    let ids: Vec<String> = all.iter().map(|x| x.img_id.clone()).collect();

    println!("all: {:#?}", ids);
}
