use std::io::Cursor;

use image::{ImageBuffer, Luma};
use linfa_linalg::qr::LeastSquaresQr;
use ndarray::{Array, Array2, ArrayView, Axis};

use crate::rocket::ROCKET;

#[derive(Debug)]
pub struct SpmImage {
    pub img_id: String,
    pub xsize: f64,
    pub ysize: f64,
    pub xres: usize,
    pub yres: usize,
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
            ImageBuffer::from_vec(self.xres as u32, self.yres as u32, pixels)
                .expect("to create image buffer");
        let rgba = img_buffer.expand_palette(&ROCKET, None);
        let mut png_bytes: Vec<u8> = Vec::new();
        rgba.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png)
            .ok();
        png_bytes
    }

    pub fn save_png(&self) {
        let out_name = format!("{}.png", self.img_id);
        let pixels = self.norm();

        image::save_buffer(
            out_name,
            &pixels,
            self.xres as u32,
            self.yres as u32,
            image::ColorType::L8,
        )
        .ok();
    }

    pub fn correct_plane(&mut self) -> &Self {
        let xres = self.xres;
        let yres = self.yres;

        let img_data = Array::from_vec(self.img_data.clone())
            .into_shape((xres, yres))
            .unwrap();
        let img_data_flat = Array::from_vec(self.img_data.clone())
            .into_shape((xres * yres, 1))
            .unwrap();
        let ones: Array2<f64> = Array::ones((xres, yres));

        let mut coeffs: Array2<f64> = Array::ones((xres * yres, 1));
        let x_coords = Array::from_shape_fn((xres, yres), |(_, j)| j as f64);
        let y_coords = Array::from_shape_fn((xres, yres), |(i, _)| i as f64);

        let x_view = ArrayView::from(&x_coords);
        let x_coords_flat = x_view.into_shape(xres * yres).unwrap();
        coeffs.push_column(x_coords_flat).unwrap();
        coeffs
            .push_column(ArrayView::from(&y_coords).into_shape(xres * yres).unwrap())
            .unwrap();
        let res = coeffs.least_squares(&img_data_flat).unwrap();
        // let qr = coeffs.qr().unwrap();
        // let res = qr.solve(&img_data_flat);

        let correction = ones * res[[0, 0]] + x_coords * res[[1, 0]] + y_coords * res[[2, 0]];
        let s = img_data - correction;

        self.img_data = s.into_raw_vec();
        self
    }

    pub fn correct_lines(&mut self) -> &Self {
        let xres = self.xres;
        let yres = self.yres;

        let img_data = Array::from_vec(self.img_data.clone())
            .into_shape((xres, yres))
            .unwrap();
        let means = img_data.mean_axis(Axis(1)).unwrap();
        let corrected = img_data - means.broadcast((yres, xres)).unwrap().t();
        self.img_data = corrected.into_raw_vec();
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
