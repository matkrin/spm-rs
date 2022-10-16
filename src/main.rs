// use mulfile_rs::read_mul;
use mulfile_rs::read_omicron_matrix;

fn main() {
    // let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    //
    // for i in mulfile {
    //     i.correct_plane().correct_lines().save_png();
    // }

    let filename = "20201111--4_1.Z_mtrx";

    // let param = get_param_info(filename);
    // let param_full = read_omicron_matrix_paramfile_full("20201111_0001.mtrx");
    let mut s = read_omicron_matrix(filename);
    println!("{:?}", s.img_data);
    s.save_png();
    // s.flip_img_data();
    // s.save_png();
    // s.iter().for_each(|x| x.save_png())
    // println!("full: {:#?}", param_full);
    // println!("parwm: {:#?}", param);
}
