// use mulfile_rs::read_mul;
use mulfile_rs::{get_param_info, read_omicron_matrix_paramfile_full, read_omicron_matrix_scanfile};

fn main() {
    // let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    //
    // for i in mulfile {
    //     i.correct_plane().correct_lines().save_png();
    // }

    let filename = "20201111--4_1.Z_mtrx";
    let param = get_param_info("20201111_0001.mtrx");
    // let param_full = read_omicron_matrix_paramfile_full("20201111_0001.mtrx");
    // let s = read_omicron_matrix_scanfile(filename);
    // println!("full: {:#?}", param_full);
    println!("param: {:#?}", param);
}
