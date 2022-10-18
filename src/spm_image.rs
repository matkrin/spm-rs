use nalgebra::{DMatrix, DVector};


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
    pub fn save_png(&self) {
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
        let pixels: Vec<u8> = self.img_data
            .iter()
            .map(|x| ((x - min) / diff * 255.0) as u8)
            .collect();
        let out_name = format!("{}.png", self.img_id);
        image::save_buffer(
            out_name,
            &pixels,
            self.xres as u32,
            self.yres as u32,
            image::ColorType::L8,
        )
        .unwrap();
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
        self.img_data = corrected.as_slice().try_into().unwrap();
        self
    }

    pub fn correct_lines(&mut self) -> &Self {
        let xres = self.xres as usize;
        let yres = self.yres as usize;

        let img_data_matrix = DMatrix::from_vec(yres, xres, self.img_data.clone());
        let means = img_data_matrix.row_mean();
        let correction = DMatrix::from_fn(yres, xres, |_, j| means[j]);
        let corrected = img_data_matrix - correction;
        self.img_data = corrected.as_slice().try_into().unwrap();
        self
    }
}
