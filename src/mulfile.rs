use std::fs::read;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::str;

use chrono::prelude::*;
use chrono::{DateTime, Utc};

use crate::spm_image::SpmImage;
use crate::utils::{read_i16_le, read_i16_le_bytes, read_i32_le, read_string};

#[derive(Debug)]
pub struct MulImage {
    pub filepath: PathBuf,
    pub img_num: i32,
    pub img_id: String,
    pub size: i32,
    pub xres: i32,
    pub yres: i32,
    pub zres: i32,
    pub datetime: DateTime<Utc>,
    pub xsize: i32,
    pub ysize: i32,
    pub xoffset: i32,
    pub yoffset: i32,
    pub zscale: i32,
    pub tilt: i32,
    pub speed: f64,
    pub line_time: f64,
    pub bias: f64,
    pub current: f64,
    pub sample: String,
    pub title: String,
    pub postpr: i32,
    pub postd1: i32,
    pub mode: i32,
    pub currfac: i32,
    pub num_pointscans: i32,
    pub unitnr: i32,
    pub version: i32,
    pub gain: i32,
    pub img_data: SpmImage,
}

// Always length 21
fn read_mul_string(cursor: &mut Cursor<&Vec<u8>>) -> String {
    read_string(cursor, 21)
}

// Image Data
fn read_mul_pixels(buffer: &[u8], zscale: i32) -> Vec<f64> {
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

fn read_mul_img_data(cursor: &mut Cursor<&Vec<u8>>, num_pixels: i32, zscale: i32) -> Vec<f64> {
    let mut buffer = vec![0; (num_pixels * 2) as usize];
    cursor.read_exact(&mut buffer).unwrap();
    read_mul_pixels(&buffer, zscale)
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

pub fn read_mul(filename: &str) -> Vec<MulImage> {
    const MUL_BLOCK: i32 = 128;
    let mut block_counter = 0;
    let mut mul: Vec<MulImage> = Vec::new();

    let bytes = read(filename).unwrap();
    let file_len = bytes.len();
    let mut cursor = Cursor::new(&bytes);

    let _nr = read_i16_le(&mut cursor);
    let adr = read_i32_le(&mut cursor);

    if adr == 3 {
        cursor
            .seek(SeekFrom::Start((adr * MUL_BLOCK) as u64))
            .expect("seeking to first block failed");
        block_counter += adr;
    } else {
        cursor
            .seek(SeekFrom::Start(0))
            .expect("seeking to start failed");
    }

    while block_counter * MUL_BLOCK < file_len as i32 {
        let img_num = read_i16_le(&mut cursor);
        println!("img num: {}", img_num);
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

        let sample = read_mul_string(&mut cursor);
        let title = read_mul_string(&mut cursor);

        let postpr = read_i16_le(&mut cursor);
        let postd1 = read_i16_le(&mut cursor);
        let mode = read_i16_le(&mut cursor);
        let currfac = read_i16_le(&mut cursor);
        let num_pointscans = read_i16_le(&mut cursor);
        let unitnr = read_i16_le(&mut cursor);
        let version = read_i16_le(&mut cursor);

        let _spare_48 = read_i16_le(&mut cursor);
        let _spare_49 = read_i16_le(&mut cursor);
        let _spare_50 = read_i16_le(&mut cursor);
        let _spare_51 = read_i16_le(&mut cursor);
        let _spare_52 = read_i16_le(&mut cursor);
        let _spare_53 = read_i16_le(&mut cursor);
        let _spare_54 = read_i16_le(&mut cursor);
        let _spare_54 = read_i16_le(&mut cursor);
        let _spare_56 = read_i16_le(&mut cursor);
        let _spare_57 = read_i16_le(&mut cursor);
        let _spare_58 = read_i16_le(&mut cursor);
        let _spare_59 = read_i16_le(&mut cursor);

        let gain = read_i16_le(&mut cursor);

        let _spare_61 = read_i16_le(&mut cursor);
        let _spare_62 = read_i16_le(&mut cursor);
        let _spare_63 = read_i16_le(&mut cursor);

        let img_data = read_mul_img_data(
            &mut cursor,
            (xres as i32 * yres as i32).into(),
            zscale.into(),
        );

        if num_pointscans > 0 {
            for _ in 0..num_pointscans {
                let _ps_size = read_i16_le(&mut cursor);
                let _ps_type = read_i16_le(&mut cursor);
                let _ps_time4scan = read_i16_le(&mut cursor);
                let _ps_minv = read_i16_le(&mut cursor);
                let _ps_maxv = read_i16_le(&mut cursor);
                let _ps_xpos = read_i16_le(&mut cursor);
                let _ps_ypos = read_i16_le(&mut cursor);
                let _ps_dz = read_i16_le(&mut cursor);
                let _ps_delay = read_i16_le(&mut cursor);
                let _ps_version = read_i16_le(&mut cursor);
                let _ps_indendelay = read_i16_le(&mut cursor);
                let _ps_xposend = read_i16_le(&mut cursor);
                let _ps_yposend = read_i16_le(&mut cursor);
                let _ps_vt_fw = read_i16_le(&mut cursor);
                let _ps_it_fw = read_i16_le(&mut cursor);
                let _ps_vt_bw = read_i16_le(&mut cursor);
                let _ps_it_bw = read_i16_le(&mut cursor);
                let _ps_lscan = read_i16_le(&mut cursor);

                let _ = cursor.seek(SeekFrom::Current((MUL_BLOCK - 18 * 2) as i64));

                let _ps_data = read_point_scan(&mut cursor, _ps_size as i32);
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
        let basename = filepath.file_stem().unwrap().to_str().unwrap();
        let img_id = format!("{}_{}", basename, img_num);

        block_counter += size as i32;

        mul.push(MulImage {
            filepath,
            img_num: img_num.into(),
            img_id: img_id.clone(),
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
            img_data: SpmImage {
                img_id,
                xres: xres as u32,
                yres: yres as u32,
                xsize: xsize as f64,
                ysize: ysize as f64,
                img_data,
            },
        })
    }
    assert_eq! {block_counter * MUL_BLOCK, bytes.len() as i32};
    mul
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_mul_string() {
        let s = "Hello this is a test!";
        assert_eq!(21, s.len());
        let buffer = s.as_bytes().to_vec();
        let mut cursor = Cursor::new(&buffer);
        assert_eq!(read_mul_string(&mut cursor), s);
    }
}
