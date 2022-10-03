use mulfile_rs::read_mul;


fn main() {
    let mulfile = read_mul("tests/stm-aarhus-mul-a.mul");

    for i in mulfile {
        i.correct_plane().correct_lines().save_png();
    }

}
