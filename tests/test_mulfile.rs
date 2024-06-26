use spm_rs::mulfile::read_mul;

const MULFILE: &str = "tests/test_files/stm-aarhus-mul-a.mul";

#[test]
fn test_current() {
    let mulfile = read_mul(MULFILE).unwrap();
    let currents: Vec<_> = mulfile.iter().map(|x| x.current).collect();
    assert_eq!(currents, vec![0.23, 0.43, 0.91, 0.65]);
}

#[test]
fn test_bias() {
    let mulfile = read_mul(MULFILE).unwrap();
    let biases: Vec<_> = mulfile.iter().map(|x| x.bias.round()).collect();
    assert_eq!(biases, vec![222.0, 293.0, 293.0, 293.0]);
}

#[test]
fn test_gain() {
    let mulfile = read_mul(MULFILE).unwrap();
    let gains: Vec<_> = mulfile.iter().map(|x| x.gain).collect();
    assert_eq!(gains, Vec::from([955; 4]));
}

#[test]
fn test_scan_duration() {
    let mulfile = read_mul(MULFILE).unwrap();
    let scan_durations: Vec<_> = mulfile.iter().map(|x| x.speed.round()).collect();
    assert_eq!(scan_durations, vec![77.0, 82.0, 123.0, 129.0]);
}

#[test]
fn test_lines_time() {
    let mulfile = read_mul(MULFILE).unwrap();
    let line_times: Vec<_> = mulfile.iter().map(|x| x.line_time.round()).collect();
    assert_eq!(line_times, vec![150.0, 160.0, 240.0, 252.0]);
}
