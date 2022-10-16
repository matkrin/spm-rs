use std::collections::HashMap;

use crate::spm_image::SpmImage;
use crate::{omicron_matrix_scan::read_omicron_matrix_scanfile, paraminfo::get_param_info};

pub fn read_omicron_matrix(filename: &str) -> SpmImage {
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

        // v_fw.push(line_fw);
        // v_bw.push(line_bw);
    }

    // v_bw.reverse();
    SpmImage{
        img_id: "forward".to_string(),
        xres: paraminfo.xres,
        yres: paraminfo.yres,
        xsize: paraminfo.xsize,
        ysize: paraminfo.ysize,
        img_data: v_fw,
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
