use spm_rs::rhk_sm4::read_rhk_sm4;

const SM4_FILE: &str = "tests/test_files/stm-rhk-sm4.SM4";

#[test]
fn test_current() {
    let sm4 = read_rhk_sm4(SM4_FILE).unwrap();
    for i in sm4 {
        assert_eq!(i.current, 1.9969940978636913 * 1e-10);
    }
}

#[test]
fn test_bias() {
    let sm4 = read_rhk_sm4(SM4_FILE).unwrap();
    for i in sm4 {
        assert_eq!(i.bias, -0.17124176025390625);
    }
}

#[test]
fn test_sizes() {
    let sm4 = read_rhk_sm4(SM4_FILE).unwrap();
    for i in sm4 {
        assert_eq!(i.xsize, 299.99998218954715 * 1e-9);
        assert_eq!(i.ysize, 299.99998218954715 * 1e-9);
    }
}

#[test]
fn test_resolutions() {
    let sm4 = read_rhk_sm4(SM4_FILE).unwrap();
    for i in sm4 {
        assert_eq!(i.xres, 512);
        assert_eq!(i.yres, 512);
    }
}

#[test]
fn test_rotation() {
    let sm4 = read_rhk_sm4(SM4_FILE).unwrap();
    for i in sm4 {
        assert_eq!(i.rotation, 116.0);
    }
}
