// use mulfile_rs::read_mul;
use mulfile_rs::read_omicron_matrix;


fn main() {
    // let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");
    //
    // for i in mulfile {
    //     i.correct_plane().correct_lines().save_png();
    // }

    read_omicron_matrix("20201111_0001.mtrx");

}
