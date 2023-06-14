use std::collections::HashMap;

use anyhow::Result;

use crate::omicron_matrix::paraminfo::get_param_info;
use crate::omicron_matrix::scanfile::read_omicron_matrix_scanfile;
use crate::spm_image::flip_img_data;
use crate::spm_image::SpmImage;

#[derive(Debug)]
pub struct OmicronMatrix {
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
    pub img_data_fw: SpmImage,
    pub img_data_bw: SpmImage,
}

pub fn read_omicron_matrix(filename: &str) -> Result<OmicronMatrix> {
    let paraminfo = get_param_info(filename)?;
    let scandata = read_omicron_matrix_scanfile(filename);
    // TODO: handle different number of images
    let _num_imgs =
        (if paraminfo.xretrace { 2 } else { 1 }) * (if paraminfo.yretrace { 2 } else { 1 });
    let mut lines = scandata.img_data.chunks(paraminfo.xres as usize);

    let mut forward_up = Vec::new();
    let mut backward_up = Vec::new();

    for _ in 0..lines.len() / 2 {
        for x in lines.next().unwrap().iter() {
            forward_up.push(tff_linear(f64::from(*x), &paraminfo.tffs))
        }

        for x in lines.next().unwrap().iter() {
            backward_up.push(tff_linear(f64::from(*x), &paraminfo.tffs))
        }
    }

    let v_fw = flip_img_data(forward_up, paraminfo.xres, paraminfo.yres);
    backward_up.reverse();

    Ok(OmicronMatrix {
        current: paraminfo.current * 1e9,
        bias: paraminfo.bias,
        xsize: paraminfo.xsize * 1e9,
        ysize: paraminfo.ysize * 1e9,
        xres: paraminfo.xres,
        yres: paraminfo.yres,
        rotation: paraminfo.rotation,
        raster_time: paraminfo.raster_time * paraminfo.xres as f64 * paraminfo.yres as f64,
        xoffset: paraminfo.xoffset * 1e9,
        yoffset: paraminfo.yoffset * 1e9,
        img_data_fw: SpmImage {
            img_id: "forward_up".to_string(),
            xres: paraminfo.xres,
            yres: paraminfo.yres,
            xsize: paraminfo.xsize,
            ysize: paraminfo.ysize,
            img_data: v_fw,
        },
        img_data_bw: SpmImage {
            img_id: "backward_up".to_string(),
            xres: paraminfo.xres,
            yres: paraminfo.yres,
            xsize: paraminfo.xsize,
            ysize: paraminfo.ysize,
            img_data: backward_up,
        },
    })
}

// TODO: datapoints seem to differ from gwyddion, there is also 'zoom' mentioned
fn tff_linear(x: f64, tffs: &HashMap<String, f64>) -> f64 {
    let offset = tffs["TFF_Linear1D.Offset [m]"];
    let factor = tffs["TFF_Linear1D.Factor [m]"];
    (x - offset) / factor
}

fn _tff_multilinear(x: f64, tffs: &HashMap<String, f64>) -> f64 {
    let raw_1 = tffs["TFF_MultiLinear1D.Raw_1 [A]"];
    let preoffset = tffs["TFF_MultiLinear1D.PreOffset [A]"];
    let offset = tffs["TFF_MultiLinear1D.Offset [A]"];
    let neutralfactor = tffs["TFF_MultiLinear1D.NeutralFactor [A]"];
    let prefactor = tffs["TFF_MultiLinear1D.PreFactor [A]"];
    (raw_1 - preoffset) * (x - offset) / neutralfactor / prefactor
}
