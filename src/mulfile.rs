use std::fs::read;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::str;

use anyhow::Result;
use chrono::prelude::*;
use chrono::{DateTime, Utc};

use crate::spm_image::flip_img_data;
use crate::spm_image::SpmImage;
use crate::utils::{read_i16_le_bytes, Bytereading};

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
fn read_mul_string(cursor: &mut Cursor<&[u8]>) -> String {
    cursor.read_string(21)
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

fn read_mul_img_data(cursor: &mut Cursor<&[u8]>, num_pixels: i32, zscale: i32) -> Vec<f64> {
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

fn read_point_scan(cursor: &mut Cursor<&[u8]>, num_data_points: i32) -> Vec<f64> {
    let mut buffer = vec![0; (num_data_points * 2) as usize];
    cursor.read_exact(&mut buffer).unwrap();
    read_data_points(&buffer)
}

pub fn read_mul(filename: &str) -> Result<Vec<MulImage>> {
    const MUL_BLOCK: i32 = 128;
    let mut block_counter = 0;
    let mut mul: Vec<MulImage> = Vec::new();

    let bytes = read(filename)?;
    let file_len = bytes.len();
    let mut cursor = Cursor::new(bytes.as_slice());

    let _nr = cursor.read_i16_le();
    let adr = cursor.read_i32_le();

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
        let img_num = cursor.read_i16_le();
        let size = cursor.read_i16_le();

        let xres = cursor.read_i16_le();
        let yres = cursor.read_i16_le();
        let zres = cursor.read_i16_le();

        let year = cursor.read_i16_le();
        let month = cursor.read_i16_le();
        let day = cursor.read_i16_le();
        let hour = cursor.read_i16_le();
        let minute = cursor.read_i16_le();
        let second = cursor.read_i16_le();

        let xsize = cursor.read_i16_le() / 10; // in nm
        let ysize = cursor.read_i16_le() / 10; // in nm

        let xoffset = cursor.read_i16_le() / 10; // in nm
        let yoffset = cursor.read_i16_le() / 10; // in nm

        let zscale = cursor.read_i16_le();
        let tilt = cursor.read_i16_le();
        let speed = cursor.read_i16_le() / 100; // in s

        let bias = cursor.read_i16_le();
        let current = cursor.read_i16_le();

        let sample = read_mul_string(&mut cursor);
        let title = read_mul_string(&mut cursor);

        let postpr = cursor.read_i16_le();
        let postd1 = cursor.read_i16_le();
        let mode = cursor.read_i16_le();
        let currfac = cursor.read_i16_le();
        let num_pointscans = cursor.read_i16_le();
        let unitnr = cursor.read_i16_le();
        let version = cursor.read_i16_le();

        let _spare_48 = cursor.read_i16_le();
        let _spare_49 = cursor.read_i16_le();
        let _spare_50 = cursor.read_i16_le();
        let _spare_51 = cursor.read_i16_le();
        let _spare_52 = cursor.read_i16_le();
        let _spare_53 = cursor.read_i16_le();
        let _spare_54 = cursor.read_i16_le();
        let _spare_54 = cursor.read_i16_le();
        let _spare_56 = cursor.read_i16_le();
        let _spare_57 = cursor.read_i16_le();
        let _spare_58 = cursor.read_i16_le();
        let _spare_59 = cursor.read_i16_le();

        let gain = cursor.read_i16_le();

        let _spare_61 = cursor.read_i16_le();
        let _spare_62 = cursor.read_i16_le();
        let _spare_63 = cursor.read_i16_le();

        let img_data = read_mul_img_data(
            &mut cursor,
            (xres as i32 * yres as i32).into(),
            zscale.into(),
        );

        if num_pointscans > 0 {
            for _ in 0..num_pointscans {
                let _ps_size = cursor.read_i16_le();
                let _ps_type = cursor.read_i16_le();
                let _ps_time4scan = cursor.read_i16_le();
                let _ps_minv = cursor.read_i16_le();
                let _ps_maxv = cursor.read_i16_le();
                let _ps_xpos = cursor.read_i16_le();
                let _ps_ypos = cursor.read_i16_le();
                let _ps_dz = cursor.read_i16_le();
                let _ps_delay = cursor.read_i16_le();
                let _ps_version = cursor.read_i16_le();
                let _ps_indendelay = cursor.read_i16_le();
                let _ps_xposend = cursor.read_i16_le();
                let _ps_yposend = cursor.read_i16_le();
                let _ps_vt_fw = cursor.read_i16_le();
                let _ps_it_fw = cursor.read_i16_le();
                let _ps_vt_bw = cursor.read_i16_le();
                let _ps_it_bw = cursor.read_i16_le();
                let _ps_lscan = cursor.read_i16_le();

                let _ = cursor.seek(SeekFrom::Current((MUL_BLOCK - 18 * 2) as i64));

                let _ps_data = read_point_scan(&mut cursor, _ps_size as i32);
            }
        }

        let line_time = f64::from(speed) / f64::from(yres) * 1000.0; // in ms
        let bias = -f64::from(bias) / 3.2768; //  in mV
        let current = f64::from(current) * f64::from(currfac) * 0.01; // in nA

        let datetime = Utc
            .ymd(year.try_into()?, month.try_into()?, day.try_into()?)
            .and_hms(hour.try_into()?, minute.try_into()?, second.try_into()?);

        let filepath = PathBuf::from(&filename);
        let basename = filepath.file_stem().unwrap().to_str().unwrap();
        let img_id = format!("{}_{}", basename, img_num);
        let img_data = flip_img_data(img_data, xres as u32, yres as u32);

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
    Ok(mul)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_mul_string() {
        let s = "Hello this is a test!";
        assert_eq!(21, s.len());
        let buffer = s.as_bytes().to_vec();
        let mut cursor = Cursor::new(buffer.as_slice());
        assert_eq!(read_mul_string(&mut cursor), s);
    }
}
