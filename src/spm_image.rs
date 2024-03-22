use std::io::Cursor;

use image::{ImageBuffer, Luma};
use nalgebra::{DMatrix, DVector};

use crate::rocket::ROCKET;

#[derive(Debug)]
pub struct SpmImage {
    pub img_id: String,
    pub xsize: f64,
    pub ysize: f64,
    pub xres: u32,
    pub yres: u32,
    pub img_data: Vec<f64>,
}

impl SpmImage {
    pub fn norm(&self) -> Vec<u8> {
        let min = self
            .img_data
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max = self
            .img_data
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let diff = max - min;
        let pixels: Vec<u8> = self
            .img_data
            .iter()
            .map(|x| ((x - min) / diff * 255.0) as u8)
            .collect();
        pixels
    }

    pub fn to_png_bytes(&self) -> Vec<u8> {
        let pixels = self.norm();
        let img_buffer: ImageBuffer<Luma<u8>, Vec<u8>> =
            ImageBuffer::from_vec(self.xres, self.yres, pixels)
                .expect("to create image buffer");
        let rgba = img_buffer.expand_palette(&ROCKET, None);
        let mut png_bytes: Vec<u8> = Vec::new();
        rgba.write_to(
            &mut Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .ok();
        png_bytes
    }

    pub fn save_png(&self) {
        let out_name = format!("{}.png", self.img_id);
        let pixels = self.norm();

        image::save_buffer(
            out_name,
            &pixels,
            self.xres,
            self.yres,
            image::ColorType::L8,
        )
        .ok();
    }

    pub fn correct_plane(&mut self) -> &Self {
        let xres = self.xres as usize;
        let yres = self.yres as usize;

        let img_data_vec = DVector::from_vec(self.img_data.clone());

        let mut coeffs = DMatrix::from_element(xres * yres, 3, 1.0);
        let x_coords = DMatrix::from_fn(yres, xres, |_, j| j as f64);
        let y_coords = DMatrix::from_fn(yres, xres, |i, _| i as f64);

        coeffs.set_column(1, &DVector::from_column_slice(x_coords.as_slice()));
        coeffs.set_column(2, &DVector::from_column_slice(y_coords.as_slice()));

        let lstsq = coeffs.svd(true, true).solve(&img_data_vec, 1e-14).unwrap();

        let ones = DMatrix::from_element(yres, xres, 1.0);

        let correction = ones * lstsq[0] + x_coords * lstsq[1] + y_coords * lstsq[2];

        let corrected = DMatrix::from_vec(yres, xres, self.img_data.clone()) - correction;
        self.img_data = corrected.as_slice().into();
        self
    }

    pub fn correct_lines(&mut self) -> &Self {
        let xres = self.xres as usize;
        let yres = self.yres as usize;

        let img_data_matrix = DMatrix::from_vec(yres, xres, self.img_data.clone());
        let means = img_data_matrix.row_mean();
        let correction = DMatrix::from_fn(yres, xres, |_, j| means[j]);
        let corrected = img_data_matrix - correction;
        self.img_data = corrected.as_slice().into();
        self
    }
}

pub fn flip_img_data(img_data: Vec<f64>, xres: u32, yres: u32) -> Vec<f64> {
    let mut flipped: Vec<f64> = Vec::with_capacity((xres * yres) as usize);
    for i in (0..yres).rev() {
        let mut line = img_data[(i * xres) as usize..((i + 1) * xres) as usize].to_owned();
        flipped.append(&mut line);
    }
    flipped
}
