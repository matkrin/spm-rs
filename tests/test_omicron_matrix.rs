use spm_rs::omicron_matrix::read_omicron_matrix;

const MTRX_FILE: &str = "tests/test_files/20201111--4_1.Z_mtrx";

#[test]
fn test_current() {
    let mtrx = read_omicron_matrix(MTRX_FILE);
    assert_eq!(mtrx.current, 0.3);
}

#[test]
fn test_bias() {
    let mtrx = read_omicron_matrix(MTRX_FILE);
    assert_eq!(format!("{:.2}", mtrx.bias), "0.60");
}

#[test]
fn test_sizes() {
    let mtrx = read_omicron_matrix(MTRX_FILE);
    assert_eq!(mtrx.xsize, 100.0);
    assert_eq!(mtrx.ysize, 100.0);
}

#[test]
fn test_resolutions() {
    let mtrx = read_omicron_matrix(MTRX_FILE);
    assert_eq!(mtrx.xres, 400);
    assert_eq!(mtrx.yres, 400);
}


#[test]
fn test_rotation() {
    let mtrx = read_omicron_matrix(MTRX_FILE);
    assert_eq!(mtrx.rotation, 0);
}

