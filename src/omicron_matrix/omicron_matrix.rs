use std::collections::HashMap;

use crate::spm_image::SpmImage;
use crate::omicron_matrix::scanfile::read_omicron_matrix_scanfile; 
use crate::omicron_matrix::paraminfo::get_param_info;
use crate::utils::flip_img_data;

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

pub fn read_omicron_matrix(filename: &str) -> OmicronMatrix {
    let paraminfo = get_param_info(filename);
    let scandata = read_omicron_matrix_scanfile(filename);
    // println!("{:#?}", paraminfo);
    let num_imgs =
        (if paraminfo.xretrace { 2 } else { 1 }) * (if paraminfo.yretrace { 2 } else { 1 });
    println!("num_imgs : {}", num_imgs);
    let mut lines = scandata.img_data.chunks(paraminfo.xres as usize);
    println!("len liens : {}", lines.len());

    // TODO: fw needs to be flipped (like mulfile)
    let mut v_fw = Vec::new();
    // TODO: bw needs to be reversed (Vec.reverse())
    let mut v_bw = Vec::new();

    for _ in 0..lines.len() / 2 {
        for x in lines.next().unwrap().iter() {
            v_fw.push(tff_linear(f64::from(*x), &paraminfo.tffs))
        }
        lines.next().unwrap().iter().for_each(|x| {
            v_bw.push(f64::from(*x))
        });
    }

    let v_fw = flip_img_data(v_fw, paraminfo.xres, paraminfo.yres);
    v_bw.reverse();

    OmicronMatrix{
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
        img_data_fw: SpmImage{
            img_id: "forward".to_string(),
            xres: paraminfo.xres,
            yres: paraminfo.yres,
            xsize: paraminfo.xsize,
            ysize: paraminfo.ysize,
            img_data: v_fw,
        },
        img_data_bw: SpmImage{
            img_id: "backward".to_string(),
            xres: paraminfo.xres,
            yres: paraminfo.yres,
            xsize: paraminfo.xsize,
            ysize: paraminfo.ysize,
            img_data: v_bw,
        },
    }
}

// TODO: datapoints seem to differ from gwyddion, there is also 'zoom' mentioned
fn tff_linear(x: f64, tffs: &HashMap<String, f64>) -> f64 {
    let offset = tffs["TFF_Linear1D.Offset [m]"];
    // println!("offset: {}", offset);
    let factor = tffs["TFF_Linear1D.Factor [m]"];
    // println!("factor: {}", factor);
    (x - tffs["TFF_Linear1D.Offset [m]"]) / tffs["TFF_Linear1D.Factor [m]"]
}

// tff multilinear
// (raw_1 - PreOffset) * (x - Offset) / NeutralFactor / PreFactor
