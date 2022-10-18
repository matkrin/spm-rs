use spm_rs::mulfile::read_mul;
use spm_rs::omicron_matrix::read_omicron_matrix;

fn main() {
    let mulfile = read_mul("tests/test_files/stm-aarhus-mul-a.mul");

    for mut i in mulfile {
        i.img_data.correct_plane();
        i.img_data.correct_lines();
        i.img_data.save_png();
    }

    let mut omicron_matrix = read_omicron_matrix("tests/test_files/20201111--4_1.Z_mtrx");
    println!("current: {}", omicron_matrix.current);
    println!("bias: {}", omicron_matrix.bias);
    println!("xsize: {}", omicron_matrix.xsize);
    println!("ysize: {}", omicron_matrix.ysize);
    println!("xres: {}", omicron_matrix.xres);
    println!("yres: {}", omicron_matrix.yres);
    println!("rotation: {}", omicron_matrix.rotation);
    omicron_matrix.img_data_fw.save_png();
    omicron_matrix.img_data_bw.save_png();
    // omicron_matrix.correct_plane();
    // omicron_matrix.correct_lines();
    // omicron_matrix.save_png();
}
