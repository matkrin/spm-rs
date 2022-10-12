// use mulfile_rs::read_mul;
// use mulfile_rs::read_omicron_matrix_paramfile;
use mulfile_rs::read_omicron_matrix_scanfile;


fn main() {
    // let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    //
    // for i in mulfile {
    //     i.correct_plane().correct_lines().save_png();
    // }

    // read_omicron_matrix_paramfile("20201111_0001.mtrx");
    read_omicron_matrix_scanfile("20201111--4_1.Z_mtrx");

}
