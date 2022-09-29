use mulfile_rs::read_mul;

fn main() {
    let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    let ids: Vec<String> = mulfile.iter().map(|x| x.img_id.clone()).collect();
    println!("Image IDs: {:#?}", ids);

    for i in mulfile {
        i.save_png();
    }
    // let img_data = &mulfile[0].img_data;
    
    // let min = img_data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    // let max = img_data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    // println!("min: {}", min);
    // println!("max: {}", max);
    // let diff = max - min;
    // println!("diff: {}", diff);
    //     // const grey = 255 - Math.round(((image.imgData[i] - min) / diff) * 255);
    // // let new: Vec<u8> = img_data.iter().rev().map(|x| ((x - min) / diff * 255.0) as u8).collect();
    // let new: Vec<u8> = img_data.iter().rev().map(|x| ((x - min) / diff * 255.0) as u8).collect();
    // image::save_buffer("image.png", &new, mulfile[0].xres as u32, mulfile[0].yres as u32, image::ColorType::L8).unwrap();
    // println!("{:?}", img_data[0]);
    // println!("{:?}", new[0]);
}
